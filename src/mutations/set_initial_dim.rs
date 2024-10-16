use std::sync::Arc;

use crate::handlers::{spawn_dim_thread, SpawnDimThreadOptions};
use hyprland::keyword::Keyword;

use crate::{cli::Cli, queries::is_single, state::LiveState};

pub fn set_initial_dim(live: &LiveState, cli: &Cli) -> anyhow::Result<()> {
    if is_single() {
        Keyword::set("decoration:dim_strength", 0)?;
        Keyword::set("decoration:dim_inactive", "yes")?;
        return Ok(())
    }

    Ok(spawn_dim_thread(SpawnDimThreadOptions {
        num_threads: Arc::clone(&live.num_threads),
        is_set_dim: Arc::clone(&live.is_set_dim),
        strength: cli.strength,
        persist: cli.persist,
        duration: cli.duration,
        first_run: true,
    }))
}
