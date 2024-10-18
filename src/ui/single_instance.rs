use crate::utils::log;
use single_instance::SingleInstance;
use std::process;

pub fn single_instance() {
    let instance = SingleInstance::new("hyprdim").unwrap();

    // Don't allow more than one hyprdim instance to run
    if !instance.is_single() {
        println!("hyprdim is already running. Use `killall hyprdim` to stop any existing processes.");

        process::exit(1);
    };

    log("hyprdim is now running.");
}
