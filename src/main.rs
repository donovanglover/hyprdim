use clap::Parser;
use cli::Cli;
use mutations::set_animation;
use mutations::set_initial_dim;
use queries::is_floating;
use queries::is_special;
use hyprdim::log;
use mutations::set_dim;
use hyprdim::spawn_dim_thread;
use queries::is_single;
use hyprland::data::Workspace;
use hyprland::event_listener::{EventListener, WindowEventData};
use hyprland::keyword::Keyword;
use hyprland::prelude::*;
use state::DimState;
use state::LiveState;
use ui::single_instance;
use std::sync::atomic::Ordering;
use ui::ctrlc;

mod cli;
mod queries;
mod mutations;
mod ui;
mod state;

fn main() -> anyhow::Result<()> {
    single_instance();

    let state = DimState::new()?;

    let cli = Cli::parse();

    let Cli {
        fade,
        ref bezier,
        strength,
        duration,
        persist,
        no_dim_when_only,
        ignore_entering_special,
        ignore_leaving_special,
        dialog_dim,
        ..
    } = cli;

    set_animation(fade, bezier)?;

    let mut event_listener = EventListener::new();

    let live = LiveState::new();

    set_initial_dim(&live, &cli)?;

    // On active window changes
    event_listener.add_active_window_change_handler(move |data| {
        // Ignore the event if no window_address was given
        let Some(WindowEventData { window_address, window_class, .. }) = data else { return };

        // Clone inside since primitives don't implement copy
        let num_threads = live.num_threads.clone();
        let is_set_dim = live.is_set_dim.clone();

        // If the last address is the same as the new window, don't dim
        if let Some(ref old_address) = *live.last_address.lock().unwrap() {
            if format!("{old_address}") == format!("{window_address}") {
                return;
            }
        }

        let mut same_class = false;

        if let Some(ref old_class) = *live.last_class.lock().unwrap() {
            if *old_class == window_class {
                same_class = true;
            }
        }

        *live.last_address.lock().unwrap() = Some(window_address.clone());
        *live.last_class.lock().unwrap() = Some(window_class.clone());

        // Get the state of the current parent workspace
        let parent_workspace = Workspace::get_active().unwrap();
        let parent_workspace_window = &parent_workspace.last_window;

        let mut same_workspace = false;

        if let Some(ref old_workspace) = *live.last_workspace.lock().unwrap() {
            if old_workspace.id == parent_workspace.id {
                same_workspace = true;
            }
        }

        *live.last_workspace.lock().unwrap() = Some(parent_workspace.clone());

        // If the parent_workspace_window is NOT the same as the window_address, then we're in a special workspace
        let is_special_workspace =
            format!("{parent_workspace_window}") != format!("0x{window_address}");

        // Keep track of being inside special workspaces and don't dim when entering them
        if is_special_workspace && !live.in_special_workspace.load(Ordering::Relaxed) {
            live.in_special_workspace.store(true, Ordering::Relaxed);

            if ignore_entering_special {
                log("info: Special workspace was opened, so not dimming.");
                return;
            }
        }

        if !is_special_workspace {
            let was_in_special = live.in_special_workspace.load(Ordering::Relaxed);

            live.in_special_workspace.store(false, Ordering::Relaxed);

            // If we're exiting for the first time, don't dim
            if ignore_leaving_special && was_in_special {
                log("info: Leaving special workspace, so not dimming.");
                return;
            }
        }

        // Enable dim when using a floating window with the same class as the last window,
        // but only if the user specified the argument to do so.
        if let Some(dialog_strength) = dialog_dim {
            if same_workspace && same_class && is_floating() {
                is_set_dim.store(true, Ordering::Relaxed);
                set_dim(dialog_strength, persist).unwrap();
                return;
            }
        }

        is_set_dim.store(false, Ordering::Relaxed);

        // Don't dim when switching to another workspace with only one window
        if no_dim_when_only {
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

        spawn_dim_thread(num_threads, is_set_dim, strength, persist, duration, false);
    });

    ctrlc(state);

    Ok(event_listener.start_listener()?)
}
