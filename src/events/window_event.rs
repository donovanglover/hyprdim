use hyprland::event_listener::{EventListener, WindowEventData};
use crate::cli::Cli;
use crate::mutations::{set_dim, set_initial_dim};
use crate::queries::{get_parent, is_floating};
use crate::state::LiveState;
use std::sync::atomic::Ordering;

pub fn window_event(live: LiveState, cli: Cli) -> anyhow::Result<()> {
    let mut event_listener = EventListener::new();

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

    event_listener.start_listener()?;

    Ok(())
}
