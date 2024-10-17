use hyprland::keyword::Keyword;
use std::sync::atomic::{AtomicBool, AtomicU16, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::thread::spawn;
use std::time::Duration;

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
/// enough, dimming is removed.
pub fn spawn_dim_thread(options: SpawnDimThreadOptions) {
    spawn(move || -> hyprland::Result<()> {
        Keyword::set("decoration:dim_inactive", "yes")?;
        Keyword::set("decoration:dim_strength", options.strength)?;

        log("info: Applied dim (new thread)");

        options.num_threads.fetch_add(1, Ordering::Relaxed);
        sleep(Duration::from_millis(options.duration));
        options.num_threads.fetch_sub(1, Ordering::Relaxed);

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
