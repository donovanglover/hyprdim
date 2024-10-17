use hyprland::keyword::Keyword;
use std::sync::atomic::{AtomicBool, AtomicU16, Ordering};
use std::sync::Arc;
use std::{thread, time};

use crate::utils::log;

pub struct SpawnDimThreadOptions {
    pub num_threads: Arc<AtomicU16>,
    pub is_set_dim: Arc<AtomicBool>,
    pub strength: f64,
    pub duration: u64,
}

/// Spawns a new thread in charge of dimming inactive windows with Hyprland.
///
/// When there are no more threads left to wait for, that is, when the user has been inactive long
/// enough, dimming is disabled.
pub fn spawn_dim_thread(options: SpawnDimThreadOptions) {
    thread::spawn(move || -> hyprland::Result<()> {
        Keyword::set("decoration:dim_inactive", "yes")?;

        // Note that dim_strength is used instead of toggling dim_inactive for smooth animations
        Keyword::set("decoration:dim_strength", options.strength)?;

        log("info: Applied dim (new thread)");

        // Wait X milliseconds, keeping track of the number of waiting threads
        options.num_threads.fetch_add(1, Ordering::Relaxed);
        thread::sleep(time::Duration::from_millis(options.duration));
        options.num_threads.fetch_sub(1, Ordering::Relaxed);

        // If this is the last thread and we're not setting dim, remove dim
        if options.num_threads.load(Ordering::Relaxed) == 0 {
            if options.is_set_dim.load(Ordering::Relaxed) {
                log("info: Last thread, but not removing dim since permanent dim is active");
            } else {
                Keyword::set("decoration:dim_strength", 0)?;

                log("info: Removed dim (last thread)");
            }
        }

        Ok(())
    });
}
