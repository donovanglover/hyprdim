use events::window_event;
use handlers::maybe_dim;
use mutations::set_animation;
use state::GlobalState;
use state::InitialState;
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

    ctrlc(initial_state);

    set_animation(options.fade, &options.bezier)?;
    maybe_dim(&global_state, &options)?;
    window_event(global_state, options)?;

    Ok(())
}
