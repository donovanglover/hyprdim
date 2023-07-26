use hyprland::event_listener::EventListenerMutable as EventListener;
use hyprland::keyword::*;
use hyprland::shared::Address;
use std::{thread, time};
pub mod cli;
use clap::Parser;
use cli::Cli;
use ctrlc;
use std::process::exit;
use std::sync::mpsc::channel;

// (1): Keep track of how many threads are running
static mut I: i32 = 0;

// (2): Keep track of the last window address
static mut ADDRESS: Option<Address> = None;

// (3): Keep track of initial variables
static mut DIM_STRENGTH: f64 = 0.0;
static mut DIM_INACTIVE: i64 = 0;

// (4): Keep track of CLI variables
static mut STRENGTH: f64 = 0.0;
static mut DURATION: u64 = 0;

fn dim() {
    unsafe {
        // Note that dim_strength is used instead of toggling dim_inactive for smooth animations
        let _ = Keyword::set("decoration:dim_strength", STRENGTH);

        // (1): Wait X milliseconds, keeping track of the number of waiting threads with I
        I += 1;
        thread::sleep(time::Duration::from_millis(DURATION));
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
                let new_address = format!("{:?}", window_address);

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

fn log_default() -> hyprland::Result<()> {
    unsafe {
        DIM_STRENGTH = match Keyword::get("decoration:dim_strength")?.value {
            OptionValue::Float(i) => i,
            _ => 0.5,
        };

        DIM_INACTIVE = match Keyword::get("decoration:dim_inactive")?.value {
            OptionValue::Int(i) => i,
            _ => 0,
        };
    }

    Ok(())
}

fn handle_termination() {
    let (tx, rx) = channel();

    ctrlc::set_handler(move || tx.send(()).expect("Could not send signal on channel."))
        .expect("Error setting Ctrl-C handler");

    rx.recv().expect("Could not receive from channel.");

    unsafe {
        let _ = Keyword::set("decoration:dim_strength", DIM_STRENGTH);
        let _ = Keyword::set("decoration:dim_inactive", DIM_INACTIVE);
    }

    exit(0);
}

fn main() -> hyprland::Result<()> {
    let _ = log_default();

    let cli = Cli::parse();

    unsafe {
        STRENGTH = cli.strength;
        DURATION = cli.duration;
    }

    let _ = Keyword::set("decoration:dim_inactive", "yes");

    let result = format!("{}{}{}{}", "fadeDim,1,", cli.fade, ",", cli.bezier);

    let _ = Keyword::set("animation", result);

    let mut event_listener = EventListener::new();

    // On active window changes
    event_listener.add_active_window_change_handler(|data, _| {
        let Some(hyprland::event_listener::WindowEventData { window_address, .. }) = data else {
            // Ignore the event if no window_address was given
            return
        };

        // Only dim if the active window is a new window
        if is_new_window(window_address) {
            let _ = thread::spawn(dim);
        }
    });

    thread::spawn(handle_termination);

    event_listener.start_listener()
}
