use clap::Parser;
use cli::Cli;
use single_instance::SingleInstance;
use std::sync::mpsc;
use std::{process, thread};
use hyprland::State;

pub mod cli;
pub mod hyprland;

/// Main function in charge of hyprdim flow logic.
///
/// Although it's possible to test all expected functionality and any regressions over time,
/// the current implementation would require an existing Hyprland environment with test
/// applications that can be used to simulate windows.
fn main() {
    let instance = SingleInstance::new("hyprdim").unwrap();

    // Don't allow more than one hyprdim instance to run
    if !instance.is_single() {
        cli::log("hyprdim is already running. Use `killall hyprdim` to stop any existing processes.");

        process::exit(1);
    };

    let state = State::new(Cli::parse()).unwrap();

    state.init();

    cli::log("hyprdim is now running.");

    // Gracefully handle hyprdim termination
    thread::spawn(move || {
        let (tx, rx) = mpsc::channel();

        ctrlc::set_handler(move || tx.send(()).expect("Could not send signal on channel."))
            .expect("Error setting Ctrl-C handler");

        rx.recv().expect("Could not receive from channel.");

        cli::log("\nhyprdim terminated successfully.");

        process::exit(0);
    });

    state.listen();
}
