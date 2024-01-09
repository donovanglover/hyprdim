use hyprland::data::{Client, WorkspaceBasic, Workspaces, Workspace};
use hyprland::event_listener::{EventListener, WindowEventData};
use hyprland::keyword::{Keyword, OptionValue};
use hyprland::prelude::*;
use hyprland::shared::Address;
use std::sync::{Arc, Mutex};
use std::{thread, time};

use crate::cli::{Cli, log};

#[derive(Debug, Clone)]
pub struct State {
    // Hyprland state
    dim_strength: f64,
    dim_inactive: i64,

    // CLI options
    strength: f64,
    persist: bool,
    fade: u8,
    bezier: String,
    duration: u64,
    ignore_entering_special: bool,
    ignore_leaving_special: bool,
    dialog_dim: Option<f64>,
    no_dim_when_only: bool,

    // Multi-threaded state
    num_threads: Arc<Mutex<u16>>,
    last_address: Arc<Mutex<Option<Address>>>,
    last_class: Arc<Mutex<Option<String>>>,
    last_workspace: Arc<Mutex<Option<Workspace>>>,
    is_set_dim: Arc<Mutex<bool>>,
    in_special_workspace: Arc<Mutex<bool>>,

}

impl State {
    pub fn new(cli: Cli) -> hyprland::Result<Self> {
        // Save dim_strength and dim_inactive values so they can be restored later
        let dim_strength = match Keyword::get("decoration:dim_strength")?.value {
            OptionValue::Float(i) => i,
            _ => 0.5,
        };

        let dim_inactive = match Keyword::get("decoration:dim_inactive")?.value {
            OptionValue::Int(i) => i,
            _ => 0,
        };

        // Keep track of state
        Ok(Self {
            dim_strength,
            dim_inactive,

            strength: cli.strength,
            persist: cli.persist,
            fade: cli.fade,
            bezier: cli.bezier,
            duration: cli.duration,
            ignore_entering_special: cli.ignore_entering_special,
            ignore_leaving_special: cli.ignore_leaving_special,
            dialog_dim: cli.dialog_dim,
            no_dim_when_only: cli.no_dim_when_only,

            num_threads: Arc::new(Mutex::new(0)),
            last_address: Arc::new(Mutex::new(None)),
            last_class: Arc::new(Mutex::new(None)),
            last_workspace: Arc::new(Mutex::new(None)),
            is_set_dim: Arc::new(Mutex::new(false)),
            in_special_workspace: Arc::new(Mutex::new(is_special()?)),
        })
    }

    pub fn init(&'static self) -> hyprland::Result<()> {
        // Set initial dim animation
        Keyword::set("animation", format!("fadeDim,1,{},{}", self.fade, self.bezier))?;

        // Initialize with dim so the user sees something, but only if the user wants dim
        if is_special()? && (self.ignore_entering_special || self.no_dim_when_only) && special_only_has_one_visible_window()? {
            Keyword::set("decoration:dim_strength", 0)?;
            Keyword::set("decoration:dim_inactive", "yes")?;
        } else {
            self.spawn_dim_thread(true);
        }

        Ok(())
    }

    /// Spawns a new thread in charge of dimming inactive windows with Hyprland.
    ///
    /// When there are no more threads left to wait for, that is, when the user has been inactive long
    /// enough, dimming is disabled.
    fn spawn_dim_thread(&'static self, first_run: bool) {
        thread::spawn(move || -> hyprland::Result<()> {
            if self.persist || first_run {
                Keyword::set("decoration:dim_inactive", "yes")?;
            };

            // Note that dim_strength is used instead of toggling dim_inactive for smooth animations
            Keyword::set("decoration:dim_strength", self.strength)?;

            log("info: Applied dim (new thread)");

            // Wait X milliseconds, keeping track of the number of waiting threads
            *self.num_threads.lock().unwrap() += 1;
            thread::sleep(time::Duration::from_millis(self.duration));
            *self.num_threads.lock().unwrap() -= 1;

            // If this is the last thread and we're not setting dim, remove dim
            if *self.num_threads.lock().unwrap() == 0 {
                if *self.is_set_dim.lock().unwrap() {
                    log("info: Last thread, but not removing dim since permanent dim is active");
                } else {
                    Keyword::set("decoration:dim_strength", 0)?;

                    log("info: Removed dim (last thread)");
                }
            }

            Ok(())
        });
    }

    pub fn restore(&self) -> hyprland::Result<()> {
        Keyword::set("decoration:dim_strength", self.dim_strength)?;
        Keyword::set("decoration:dim_inactive", self.dim_inactive)?;

        Ok(())
    }

    pub fn listen(&'static self) {
        let mut event_listener = EventListener::new();

        // On active window changes
        event_listener.add_active_window_change_handler(|data| {
            // Ignore the event if no window_address was given
            let Some(WindowEventData { window_address, window_class, .. }) = data else { return };

            // If the last address is the same as the new window, don't dim
            if let Some(ref old_address) = *self.last_address.lock().unwrap() {
                if format!("{old_address}") == format!("{window_address}") {
                    return;
                }
            }

            let mut same_class = false;

            if let Some(ref old_class) = *self.last_class.lock().unwrap() {
                if *old_class == window_class {
                    same_class = true;
                }
            }

            *self.last_address.lock().unwrap() = Some(window_address.clone());
            *self.last_class.lock().unwrap() = Some(window_class.clone());

            // Get the state of the current parent workspace
            let parent_workspace = Workspace::get_active().unwrap();
            let parent_workspace_window = &parent_workspace.last_window;

            let mut same_workspace = false;

            if let Some(ref old_workspace) = *self.last_workspace.lock().unwrap() {
                if old_workspace.id == parent_workspace.id {
                    same_workspace = true;
                }
            }

            *self.last_workspace.lock().unwrap() = Some(parent_workspace.clone());

            // If the parent_workspace_window is NOT the same as the window_address, then we're in a special workspace
            let is_special_workspace =
                format!("{parent_workspace_window}") != format!("0x{window_address}");

            // Keep track of being inside special workspaces and don't dim when entering them
            if is_special_workspace && !*self.in_special_workspace.lock().unwrap() {
                *self.in_special_workspace.lock().unwrap() = true;

                if self.ignore_entering_special {
                    log("info: Special workspace was opened, so not dimming.");
                    return;
                }
            }

            if !is_special_workspace {
                let was_in_special = *self.in_special_workspace.lock().unwrap();

                *self.in_special_workspace.lock().unwrap() = false;

                // If we're exiting for the first time, don't dim
                if self.ignore_leaving_special && was_in_special {
                    log("info: Leaving special workspace, so not dimming.");
                    return;
                }
            }

            // Enable dim when using a floating window with the same class as the last window,
            // but only if the user specified the argument to do so.
            if let Some(dialog_strength) = self.dialog_dim {
                if same_workspace && same_class && is_floating().is_ok_and(|x| x == true) {
                    *self.is_set_dim.lock().unwrap() = true;
                    set_dim(dialog_strength, self.persist).unwrap();
                    return;
                }
            }

            *self.is_set_dim.lock().unwrap() = false;

            // Don't dim when switching to another workspace with only one window
            if self.no_dim_when_only {
                if (parent_workspace.windows == 1 || parent_workspace.fullscreen)
                    && !is_special_workspace
                {
                    Keyword::set("decoration:dim_strength", 0).unwrap();
                    log("info: Parent workspace only has one window or that window is fullscreen, so not dimming.");
                    return;
                }

                if is_special().is_ok_and(|x| x == true) && special_only_has_one_visible_window().is_ok_and(|x| x == true) {
                    Keyword::set("decoration:dim_strength", 0).unwrap();
                    log("info: Special workspace only has one window, so not dimming.");
                    return;
                }
            }

            self.spawn_dim_thread(false);
        });

        event_listener.start_listener();
    }
}

/// Gets whether the current workspace is a special workspace or not.
///
/// This function works by getting which workspace the active window is in.
///
/// The if statement is used to make sure this function works when no window
/// is the active window.
pub fn is_special() -> hyprland::Result<bool> {
    if let Some(client) = Client::get_active()? {
        let Client { workspace, .. } = client;
        return Ok(workspace.name.contains("special"));
    }

    Ok(false)
}

/// Returns true if there is only one visible window in the special workspace.
///
/// In the future, this function should be updated to accommodate for fullscreen
/// windows in special workspaces if Hyprland implements it.
///
/// https://github.com/hyprwm/Hyprland/issues/2173
pub fn special_only_has_one_visible_window() -> hyprland::Result<bool> {
    if let Some(client) = Client::get_active()? {
        let Client { workspace, .. } = client;
        let WorkspaceBasic { id, .. } = workspace;

        for workspace in Workspaces::get()? {
            if workspace.id == id {
                return Ok(workspace.windows == 1);
            }
        }
    }

    Ok(false)
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

/// Checks if the active window is floating or not.
///
/// Returns false if no window is active.
pub fn is_floating() -> hyprland::Result<bool> {
    if let Some(client) = Client::get_active()? {
        let Client { floating, .. } = client;
        return Ok(floating);
    }

    Ok(false)
}
