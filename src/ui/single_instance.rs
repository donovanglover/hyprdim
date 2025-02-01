use crate::utils::log;
use single_instance::SingleInstance;
use std::process;

pub fn single_instance() {
    let instance = Box::new(SingleInstance::new("hyprdim").unwrap());

    if instance.is_single() {
        Box::leak(instance);

        log("hyprdim is now running.");

        return;
    }

    println!(
        "hyprdim is already running. Use `killall hyprdim` to stop any existing processes."
    );

    process::exit(1);
}
