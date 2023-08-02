use hyprland::event_listener::EventListener;
use hyprland::keyword::{Keyword, OptionValue};
use hyprland::shared::Address;
use std::{thread, time, process};
use clap::Parser;
use cli::Cli;
use std::sync::{Arc, Mutex, mpsc};

mod cli;

fn main() -> hyprland::Result<()> {
    let dim_strength = match Keyword::get("decoration:dim_strength")?.value {
        OptionValue::Float(i) => i,
        _ => 0.5,
    };

    let dim_inactive = match Keyword::get("decoration:dim_inactive")?.value {
        OptionValue::Int(i) => i,
        _ => 0,
    };

    let cli = Cli::parse();

    Keyword::set("decoration:dim_inactive", "yes")?;
    Keyword::set("animation", format!("{}{}{}{}", "fadeDim,1,", cli.fade, ",", cli.bezier))?;

    let mut event_listener = EventListener::new();

    let num_threads_outer = Arc::new(Mutex::new(0));
    let last_address_outer: Arc<Mutex<Option<Address>>> = Arc::new(Mutex::new(None));

    // On active window changes
    event_listener.add_active_window_change_handler(move |data| {
        let Some(hyprland::event_listener::WindowEventData { window_address, .. }) = data else {
            // Ignore the event if no window_address was given
            return
        };

        let num_threads = num_threads_outer.clone();
        let last_address = last_address_outer.clone();

        // If the last address is the same as the new window, don't dim
        if let Some(ref address) = *last_address.lock().unwrap() {
            let old_address = format!("{:?}", address.clone());
            let new_address = format!("{:?}", window_address);

            if old_address == new_address {
                return
            }
        }

        *last_address.lock().unwrap() = Some(window_address);

        thread::spawn(move || -> hyprland::Result<()> {
            if cli.persist {
                Keyword::set("decoration:dim_inactive", "yes")?;
            };

            // Note that dim_strength is used instead of toggling dim_inactive for smooth animations
            Keyword::set("decoration:dim_strength", cli.strength)?;

            // Wait X milliseconds, keeping track of the number of waiting threads
            *num_threads.lock().unwrap() += 1;
            thread::sleep(time::Duration::from_millis(cli.duration));
            *num_threads.lock().unwrap() -= 1;

            // If this is the last thread, remove dim
            if *num_threads.lock().unwrap() == 0 {
                Keyword::set("decoration:dim_strength", 0)?;
            }

            Ok(())
        });
    });

    thread::spawn(move || {
        let (tx, rx) = mpsc::channel();

        ctrlc::set_handler(move || tx.send(()).expect("Could not send signal on channel."))
            .expect("Error setting Ctrl-C handler");

        rx.recv().expect("Could not receive from channel.");

        let _ = Keyword::set("decoration:dim_strength", dim_strength);
        let _ = Keyword::set("decoration:dim_inactive", dim_inactive);

        process::exit(0);
    });

    event_listener.start_listener()
}
