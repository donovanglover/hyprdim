use clap::Parser;
use cli::Cli;
use hyprland::data::{Client, WorkspaceBasic, Workspaces};
use hyprland::keyword::Keyword;
use hyprland::prelude::*;
use std::sync::{Arc, Mutex};
use std::{thread, time};

mod cli;

/// A helper function to only print what's happening to users if they enable the verbose flag.
pub fn log(text: &str) {
    let Cli { verbose, .. } = Cli::parse();

    if verbose {
        println!("{text}")
    }
}

/// Spawns a new thread in charge of dimming inactive windows with Hyprland.
///
/// When there are no more threads left to wait for, that is, when the user has been inactive long
/// enough, dimming is disabled.
pub fn spawn_dim_thread(
    num_threads: Arc<Mutex<u16>>,
    strength: f64,
    persist: bool,
    duration: u64,
    first_run: bool,
) {
    thread::spawn(move || -> hyprland::Result<()> {
        if persist || first_run {
            Keyword::set("decoration:dim_inactive", "yes")?;
        };

        // Note that dim_strength is used instead of toggling dim_inactive for smooth animations
        Keyword::set("decoration:dim_strength", strength)?;

        log("info: Applied dim (new thread)");

        // Wait X milliseconds, keeping track of the number of waiting threads
        *num_threads.lock().unwrap() += 1;
        thread::sleep(time::Duration::from_millis(duration));
        *num_threads.lock().unwrap() -= 1;

        // If this is the last thread, remove dim
        if *num_threads.lock().unwrap() == 0 {
            Keyword::set("decoration:dim_strength", 0)?;

            log("info: Removed dim (last thread)");
        }

        Ok(())
    });
}

/// Gets whether the current workspace is a special workspace or not.
///
/// This function works by getting which workspace the active window is in.
pub fn is_special() -> bool {
    let Client { workspace, .. } = Client::get_active().unwrap().unwrap();

    workspace.name.contains("special")
}

/// Returns true if there is only one visible window in the special workspace.
///
/// In the future, this function should be updated to accommodate for fullscreen
/// windows in special workspaces if Hyprland implements it.
///
/// https://github.com/hyprwm/Hyprland/issues/2173
pub fn special_only_has_one_visible_window() -> bool {
    let Client { workspace, .. } = Client::get_active().unwrap().unwrap();
    let WorkspaceBasic { id, .. } = workspace;

    for workspace in Workspaces::get().unwrap() {
        if workspace.id == id {
            return workspace.windows == 1;
        }
    }

    false
}
