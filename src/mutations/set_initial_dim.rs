use hyprdim::spawn_dim_thread;
use hyprland::keyword::Keyword;

use crate::{cli::Cli, queries::is_single, state::LiveState};

pub fn set_initial_dim(live: &LiveState, cli: &Cli) -> anyhow::Result<()> {
    if is_single() {
        Keyword::set("decoration:dim_strength", 0)?;
        Keyword::set("decoration:dim_inactive", "yes")?;
        return Ok(())
    }

    Ok(spawn_dim_thread(
        live.num_threads.clone(),
        live.is_set_dim.clone(),
        cli.strength,
        cli.persist,
        cli.duration,
        true,
    ))
}
