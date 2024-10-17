use events::window_event;
use mutations::set_animation;
use mutations::set_initial_dim;
use state::InitialState;
use state::GlobalState;
use ui::clap;
use ui::ctrlc;
use ui::hyprland_version;
use ui::single_instance;

mod cli;
mod events;
mod handlers;
mod mutations;
mod queries;
mod state;
mod ui;
mod utils;

const MINIMUM_VERSION: &str = "0.42.0";

fn main() -> anyhow::Result<()> {
    single_instance();

    if !hyprland_version(MINIMUM_VERSION)? {
        println!(
            "WARNING: This hyprdim version only supports Hyprland v{} and above.",
            MINIMUM_VERSION
        );
    }

    let initial_state = InitialState::new()?;
    let global_state = GlobalState::new();
    let options = clap();

    set_animation(cli.fade, &cli.bezier)?;
    set_initial_dim(&global_state, &cli)?;
    ctrlc(initial_state);
    window_event(global_state, cli)?;

    Ok(())
}
