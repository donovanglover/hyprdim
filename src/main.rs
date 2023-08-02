use hyprland::event_listener::EventListenerMutable as EventListener;
use hyprland::keyword::{Keyword, OptionValue};
use hyprland::shared::Address;
use std::{thread, time};
pub mod cli;
use clap::Parser;
use cli::Cli;
use ctrlc::set_handler;
use std::process::exit;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};

// (3): Keep track of initial variables
static mut DIM_STRENGTH: f64 = 0.0;
static mut DIM_INACTIVE: i64 = 0;

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

    set_handler(move || tx.send(()).expect("Could not send signal on channel."))
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

    let _ = Keyword::set("decoration:dim_inactive", "yes");

    let result = format!("{}{}{}{}", "fadeDim,1,", cli.fade, ",", cli.bezier);

    let _ = Keyword::set("animation", result);

    let mut event_listener = EventListener::new();

    let num_threads_outer = Arc::new(Mutex::new(0));
    let last_address_outer: Arc<Mutex<Option<Address>>> = Arc::new(Mutex::new(None));

    // On active window changes
    event_listener.add_active_window_change_handler(move |data, _| {
        let Some(hyprland::event_listener::WindowEventData { window_address, .. }) = data else {
            // Ignore the event if no window_address was given
            return
        };

        let num_threads = num_threads_outer.clone();
        let last_address = last_address_outer.clone();
        let mut windows_are_the_same = false;

        // If the saved address is the same as the new window, they're the same
        if let Some(ref address) = *last_address.lock().unwrap() {
            let old_address = format!("{:?}", address.clone());
            let new_address = format!("{:?}", window_address);

            if old_address == new_address {
                windows_are_the_same = true;
            }
        }

        if windows_are_the_same {
            return
        }

        *last_address.lock().unwrap() = Some(window_address);

        if cli.persist {
            let _ = Keyword::set("decoration:dim_inactive", "yes");
        };

        thread::spawn(move || {
            // Note that dim_strength is used instead of toggling dim_inactive for smooth animations
            let _ = Keyword::set("decoration:dim_strength", cli.strength);

            // Wait X milliseconds, keeping track of the number of waiting threads
            *num_threads.lock().unwrap() += 1;
            thread::sleep(time::Duration::from_millis(cli.duration));
            *num_threads.lock().unwrap() -= 1;

            // If this is the last thread, remove dim
            if *num_threads.lock().unwrap() == 0 {
                let _ = Keyword::set("decoration:dim_strength", 0);
            }
        });
    });

    thread::spawn(handle_termination);

    event_listener.start_listener()
}
