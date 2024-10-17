use handlers::dialog_dim;
use handlers::spawn_dim_thread;
use handlers::DialogDimOptions;
use handlers::SpawnDimThreadOptions;
use hyprland::data::Workspace;
use hyprland::event_listener::{EventListener, WindowEventData};
use hyprland::keyword::Keyword;
use hyprland::prelude::*;
use mutations::set_animation;
use mutations::set_initial_dim;
use queries::is_single;
use queries::is_special;
use state::DimState;
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

    let state = DimState::new()?;
    let cli = clap();
    let mut event_listener = EventListener::new();
    let live = LiveState::new();

    set_animation(cli.fade, &cli.bezier)?;
    set_initial_dim(&live, &cli)?;

    event_listener.add_active_window_change_handler(move |data| {
        let Some(WindowEventData { window_address, window_class, .. }) = data else { return };
        let num_threads = Arc::clone(&live.num_threads);
        let is_set_dim = Arc::clone(&live.is_set_dim);
        let mut same_class = false;
        let parent_workspace = Workspace::get_active().unwrap();
        let parent_workspace_window = &parent_workspace.last_window;

        if let Some(ref old_address) = *live.last_address.lock().unwrap() {
            if format!("{old_address}") == format!("{window_address}") {
                return;
            }
        }

        if let Some(ref old_class) = *live.last_class.lock().unwrap() {
            if *old_class == window_class {
                same_class = true;
            }
        }

        let is_special_workspace =
            format!("{parent_workspace_window}") != format!("0x{window_address}");

        *live.last_address.lock().unwrap() = Some(window_address);
        *live.last_class.lock().unwrap() = Some(window_class);

        let mut same_workspace = false;

        if let Some(ref old_workspace) = *live.last_workspace.lock().unwrap() {
            if old_workspace.id == parent_workspace.id {
                same_workspace = true;
            }
        }

        *live.last_workspace.lock().unwrap() = Some(parent_workspace.clone());

        // Keep track of being inside special workspaces and don't dim when entering them
        if is_special_workspace && !live.in_special_workspace.load(Ordering::Relaxed) {
            live.in_special_workspace.store(true, Ordering::Relaxed);

            if cli.ignore_entering_special {
                log("info: Special workspace was opened, so not dimming.");
                return;
            }
        }

        if !is_special_workspace {
            let was_in_special = live.in_special_workspace.load(Ordering::Relaxed);

            live.in_special_workspace.store(false, Ordering::Relaxed);

            // If we're exiting for the first time, don't dim
            if cli.ignore_leaving_special && was_in_special {
                log("info: Leaving special workspace, so not dimming.");
                return;
            }
        }

        // Enable dim when using a floating window with the same class as the last window,
        // but only if the user specified the argument to do so.
        let did_dim = dialog_dim(&cli, DialogDimOptions {
            same_class,
            same_workspace
        });

        is_set_dim.store(did_dim, Ordering::Relaxed);

        if did_dim {
            return;
        }

        // Don't dim when switching to another workspace with only one window
        if cli.no_dim_when_only {
            if (parent_workspace.windows == 1 || parent_workspace.fullscreen)
                && !is_special_workspace
            {
                Keyword::set("decoration:dim_strength", 0).unwrap();
                log("info: Parent workspace only has one window or that window is fullscreen, so not dimming.");
                return;
            }

            if is_special() && is_single() {
                Keyword::set("decoration:dim_strength", 0).unwrap();
                log("info: Special workspace only has one window, so not dimming.");
                return;
            }
        }

        spawn_dim_thread(SpawnDimThreadOptions {
            num_threads,
            is_set_dim,
            strength: cli.strength,
            duration: cli.duration,
        });
    });

    ctrlc(state);

    Ok(event_listener.start_listener()?)
}
