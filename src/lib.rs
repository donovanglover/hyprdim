use clap::Parser;
use cli::Cli;
use hyprland::keyword::Keyword;
use std::sync::{Arc, Mutex};
use std::{thread, time};

mod cli;

pub fn log(text: &str) {
    let Cli { verbose, .. } = Cli::parse();

    if verbose {
        println!("{text}")
    }
}

pub fn spawn_dim_thread(num_threads: Arc<Mutex<u16>>, strength: f64, persist: bool, duration: u64, first_run: bool) {
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
