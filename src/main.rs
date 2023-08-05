use clap::Parser;
use cli::Cli;
use hyprdim::is_special;
use hyprdim::log;
use hyprdim::num_windows_special;
use hyprdim::spawn_dim_thread;
use hyprland::data::Workspace;
use hyprland::event_listener::{EventListener, WindowEventData};
use hyprland::keyword::{Keyword, OptionValue};
use hyprland::prelude::*;
use hyprland::shared::Address;
use single_instance::SingleInstance;
use std::sync::{mpsc, Arc, Mutex};
use std::{process, thread};

mod cli;

fn main() -> hyprland::Result<()> {
    let instance = SingleInstance::new("hyprdim").unwrap();

    // Don't allow more than one hyprdim instance to run
    if !instance.is_single() {
        log("hyprdim is already running. Use `killall hyprdim` to stop any existing processes.");

        process::exit(1);
    };

    log("hyprdim is now running.");

    // Save dim_strength and dim_inactive values so they can be restored later
    let dim_strength = match Keyword::get("decoration:dim_strength")?.value {
        OptionValue::Float(i) => i,
        _ => 0.5,
    };

    let dim_inactive = match Keyword::get("decoration:dim_inactive")?.value {
        OptionValue::Int(i) => i,
        _ => 0,
    };

    let Cli {
        fade,
        bezier,
        strength,
        duration,
        persist,
        no_dim_when_only,
        ignore_entering_special,
        ignore_leaving_special,
        ..
    } = Cli::parse();

    // Set initial dim animation
    Keyword::set("animation", format!("fadeDim,1,{fade},{bezier}"))?;

    let mut event_listener = EventListener::new();

    // Keep track of state
    let num_threads_outer: Arc<Mutex<u16>> = Arc::new(Mutex::new(0));
    let last_address_outer: Arc<Mutex<Option<Address>>> = Arc::new(Mutex::new(None));
    let in_special_workspace_outer: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));

    // Initialize with dim so the user sees something, but only if the user wants dim
    if is_special() && (ignore_entering_special || (num_windows_special() == 1 && no_dim_when_only)) {
        *in_special_workspace_outer.lock().unwrap() = true;
        Keyword::set("decoration:dim_strength", 0)?;
        Keyword::set("decoration:dim_inactive", "yes")?;
    } else {
        spawn_dim_thread(num_threads_outer.clone(), strength, persist, duration, true);
    }

    // On active window changes
    event_listener.add_active_window_change_handler(move |data| {
        // Ignore the event if no window_address was given
        let Some(WindowEventData { window_address, .. }) = data else { return };

        let num_threads = num_threads_outer.clone();
        let last_address = last_address_outer.clone();
        let in_special_workspace = in_special_workspace_outer.clone();

        // If the last address is the same as the new window, don't dim
        if let Some(ref old_address) = *last_address.lock().unwrap() {
            if format!("{old_address}") == format!("{window_address}") {
                return;
            }
        }

        *last_address.lock().unwrap() = Some(window_address.clone());

        // Get the state of the current parent workspace
        let parent_workspace = Workspace::get_active().unwrap();
        let parent_workspace_window = &parent_workspace.last_window;

        // If the parent_workspace_window is NOT the same as the window_address, then we're in a special workspace
        let is_special_workspace = format!("{parent_workspace_window}") != format!("0x{window_address}");

        // Keep track of being inside special workspaces and don't dim when entering them
        if is_special_workspace && !*in_special_workspace.lock().unwrap() {
            *in_special_workspace.lock().unwrap() = true;

            if ignore_entering_special {
                log("info: Special workspace was opened, so not dimming.");
                return
            }
        }

        if !is_special_workspace {
            let was_in_special = *in_special_workspace.lock().unwrap();

            *in_special_workspace.lock().unwrap() = false;

            // If we're exiting for the first time, don't dim
            if ignore_leaving_special && was_in_special {
                log("info: Leaving special workspace, so not dimming.");
                return
            }
        }

        // Don't dim when switching to another workspace with only one window
        if no_dim_when_only {
            if (parent_workspace.windows == 1 || parent_workspace.fullscreen) && !is_special_workspace {
                log("info: Parent workspace only has one window, so not dimming.");
                return
            }
        }

        spawn_dim_thread(num_threads, strength, persist, duration, false);
    });

    thread::spawn(move || -> hyprland::Result<()> {
        let (tx, rx) = mpsc::channel();

        ctrlc::set_handler(move || tx.send(()).expect("Could not send signal on channel."))
            .expect("Error setting Ctrl-C handler");

        rx.recv().expect("Could not receive from channel.");

        Keyword::set("decoration:dim_strength", dim_strength)?;
        Keyword::set("decoration:dim_inactive", dim_inactive)?;

        log("\nhyprdim terminated successfully.");

        process::exit(0);
    });

    event_listener.start_listener()
}
