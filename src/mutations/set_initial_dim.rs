use std::sync::Arc;

use crate::handlers::{spawn_dim_thread, SpawnDimThreadOptions};

use crate::utils::log;
use crate::{cli::Cli, queries::is_single, state::GlobalState};

use super::set_dim;

pub fn set_initial_dim(global: &GlobalState, cli: &Cli) -> anyhow::Result<()> {
    if is_single() {
        set_dim(0.0)?;
        log("info: Workspace only has one window, so not dimming.");
    } else {
        spawn_dim_thread(SpawnDimThreadOptions {
            num_threads: Arc::clone(&global.num_threads),
            is_set_dim: Arc::clone(&global.is_set_dim),
            strength: cli.strength,
            duration: cli.duration,
        });
    }

    Ok(())
}