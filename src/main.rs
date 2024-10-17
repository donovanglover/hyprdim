use hyprland::event_listener::{EventListener, WindowEventData};
use mutations::set_animation;
use mutations::set_dim;
use mutations::set_initial_dim;
use queries::{get_parent, is_floating};
use state::InitialState;
use state::LiveState;
use std::sync::atomic::Ordering;
use ui::clap;
use ui::ctrlc;
use ui::hyprland_version;
use ui::single_instance;

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

        let parent_workspace = get_parent();
        let mut dialog_dim = false;

        if let Some(ref last_address) = *live.last_address.lock().unwrap() {
            if format!("{last_address}") == format!("{window_address}") {
                return;
            }
        }

        if let Some(ref last_class) = *live.last_class.lock().unwrap() {
            if *last_class == window_class {
                if let Some(ref last_workspace) = *live.last_workspace.lock().unwrap() {
                    if last_workspace.id == parent_workspace.id {
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
        *live.last_workspace.lock().unwrap() = Some(parent_workspace);

        if dialog_dim {
            return;
        }

        set_initial_dim(&live, &cli).unwrap()
    });

    ctrlc(initial_state);

    Ok(event_listener.start_listener()?)
}
