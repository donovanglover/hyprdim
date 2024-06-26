use clap::Parser;
use cli::Cli;
use hyprland::data::{Client, WorkspaceBasic, Workspaces};
use hyprland::keyword::Keyword;
use hyprland::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicU16, Ordering};
use std::sync::Arc;
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
    num_threads: Arc<AtomicU16>,
    is_set_dim: Arc<AtomicBool>,
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
        num_threads.fetch_add(1, Ordering::Relaxed);
        thread::sleep(time::Duration::from_millis(duration));
        num_threads.fetch_sub(1, Ordering::Relaxed);

        // If this is the last thread and we're not setting dim, remove dim
        if num_threads.load(Ordering::Relaxed) == 0 {
            if is_set_dim.load(Ordering::Relaxed) {
                log("info: Last thread, but not removing dim since permanent dim is active");
            } else {
                Keyword::set("decoration:dim_strength", 0)?;

                log("info: Removed dim (last thread)");
            }
        }

        Ok(())
    });
}

/// Sets the dim strength to a specific value permanently until it gets changed again.
///
/// Useful for setting the dim of dialog windows.
pub fn set_dim(strength: f64, persist: bool) -> hyprland::Result<()> {
    if persist {
        Keyword::set("decoration:dim_inactive", "yes")?;
    };

    Keyword::set("decoration:dim_strength", strength)?;

    log("info: Set a permanent dim (until next event) without spawning thread");

    Ok(())
}

/// Gets whether the current workspace is a special workspace or not.
///
/// This function works by getting which workspace the active window is in.
///
/// The if statement is used to make sure this function works when no window
/// is the active window.
pub fn is_special() -> bool {
    if let Some(client) = Client::get_active().unwrap() {
        let Client { workspace, .. } = client;
        return workspace.name.contains("special");
    }

    false
}

/// Returns true if there is only one visible window in the special workspace.
///
/// In the future, this function should be updated to accommodate for fullscreen
/// windows in special workspaces if Hyprland implements it.
///
/// https://github.com/hyprwm/Hyprland/issues/2173
pub fn special_only_has_one_visible_window() -> bool {
    if let Some(client) = Client::get_active().unwrap() {
        let Client { workspace, .. } = client;
        let WorkspaceBasic { id, .. } = workspace;

        for workspace in Workspaces::get().unwrap() {
            if workspace.id == id {
                return workspace.windows == 1;
            }
        }
    }

    false
}

/// Checks if the active window is floating or not.
///
/// Returns false if no window is active.
pub fn is_floating() -> bool {
    if let Some(client) = Client::get_active().unwrap() {
        let Client { floating, .. } = client;
        return floating;
    }

    false
}
