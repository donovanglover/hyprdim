use hyprland::event_listener::EventListenerMutable as EventListener;
use hyprland::keyword::*;
use hyprland::shared::Address;
use std::{thread, time};

// (1): Keep track of how many threads are running
static mut I: i32 = 0;

// (2): Keep track of the last window address
static mut ADDRESS: Option<Address> = None;

fn dim() {
    // Note tha dim_strength is used instead of toggling dim_inactive for smooth animations
    let _ = Keyword::set("decoration:dim_strength", 0.25);

    unsafe {
        // (1): Wait X milliseconds, keeping track of the number of waiting threads with I
        I += 1;
        thread::sleep(time::Duration::from_millis(800));
        I -= 1;

        // (1): If this is the last thread, remove dim
        if I == 0 {
            let _ = Keyword::set("decoration:dim_strength", 0);
        }
    }
}

fn is_new_window(window_address: Address) -> bool {
    let mut windows_are_the_same = false;

    unsafe {
        match &ADDRESS {
            // (2): If the saved address is the same as the new window, they're the same
            Some(address) => {
                let old_address = format!("{:?}", address.clone());
                let new_address = format!("{:?}", window_address.clone());

                if old_address == new_address {
                    windows_are_the_same = true;
                }
            }

            // (2): Fallback for when an initial address hasn't been saved yet
            None => {}
        }
        ADDRESS = Some(window_address);
    }

    !windows_are_the_same
}

fn main() -> hyprland::Result<()> {
    let _ = Keyword::set("decoration:dim_inactive", "yes");

    let mut event_listener = EventListener::new();

    // On active window changes
    event_listener.add_active_window_change_handler(|data, _| {
        let Some(hyprland::event_listener::WindowEventData { window_address, .. }) = data else {
            // Ignore the event if no window_address was given
            return
        };

        // Only dim if the active window is a new window
        if is_new_window(window_address) {
            let _ = thread::spawn(|| dim());
        }
    });

    event_listener.start_listener()
}
