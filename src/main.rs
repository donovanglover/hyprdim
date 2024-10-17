use handlers::spawn_dim_thread;
use handlers::SpawnDimThreadOptions;
use hyprland::data::Workspace;
use hyprland::event_listener::{EventListener, WindowEventData};
use hyprland::keyword::Keyword;
use hyprland::prelude::*;
use mutations::set_animation;
use mutations::set_dim;
use mutations::set_initial_dim;
use queries::is_floating;
use queries::is_single;
use state::InitialState;
use state::LiveState;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use ui::clap;
use ui::ctrlc;
use ui::hyprland_version;
use ui::single_instance;
use utils::log;

mod cli;
mod handlers;
mod mutations;
mod queries;
mod state;
mod ui;
mod utils;

const MINIMUM_VERSION: &str = "0.42.0";

fn main() -> anyhow::Result<()> {
    single_instance();

    if !hyprland_version(MINIMUM_VERSION)? {
        println!(
            "WARNING: This hyprdim version only supports Hyprland v{} and above.",
            MINIMUM_VERSION
        );
    }

    let initial_state = InitialState::new()?;
    let cli = clap();
    let mut event_listener = EventListener::new();
    let live = LiveState::new();

    set_animation(cli.fade, &cli.bezier)?;
    set_initial_dim(&live, &cli)?;

    event_listener.add_active_window_change_handler(move |data| {
        let Some(WindowEventData {
            window_address,
            window_class,
            ..
        }) = data
        else {
            return;
        };

        let top_level_workspace = Workspace::get_active().unwrap();
        let mut dialog_dim = false;

        if let Some(ref last_address) = *live.last_address.lock().unwrap() {
            if format!("{last_address}") == format!("{window_address}") {
                return;
            }
        }

        if let Some(ref last_class) = *live.last_class.lock().unwrap() {
            if *last_class == window_class {
                if let Some(ref last_workspace) = *live.last_workspace.lock().unwrap() {
                    if last_workspace.id == top_level_workspace.id {
                        if is_floating() {
                            set_dim(cli.dialog_dim).unwrap();

                            dialog_dim = true;
                        }
                    }
                }
            }
        }

        live.is_set_dim.store(dialog_dim, Ordering::Relaxed);
        *live.last_address.lock().unwrap() = Some(window_address);
        *live.last_class.lock().unwrap() = Some(window_class);
        *live.last_workspace.lock().unwrap() = Some(top_level_workspace);

        if dialog_dim {
            return;
        }

        if is_single() {
            Keyword::set("decoration:dim_strength", 0).unwrap();
            log("info: Workspace only has one window, so not dimming.");
            return;
        }

        spawn_dim_thread(SpawnDimThreadOptions {
            num_threads: Arc::clone(&live.num_threads),
            is_set_dim: Arc::clone(&live.is_set_dim),
            strength: cli.strength,
            duration: cli.duration,
        });
    });

    ctrlc(initial_state);

    Ok(event_listener.start_listener()?)
}
