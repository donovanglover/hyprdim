use clap::Parser;

use crate::cli::styles;
use crate::cli::ABOUT;

#[derive(Parser)]
#[command(author, version, about, long_about = ABOUT, styles = styles())]
pub struct Options {
    /// A value from 0 (no dim) to 1 (maximum dim)
    ///
    /// Note that negative numbers such as -1 and -5 are also supported for "light dim".
    #[arg(short, long, default_value_t = 0.4)]
    pub strength: f64,

    /// How many milliseconds to wait before removing dim
    #[arg(short, long, default_value_t = 800)]
    pub duration: u64,

    /// Fade animation speed from 0 (instantaneous) to 255 (very slow)
    ///
    /// The slower the fade animation speed is, the higher the duration should be before
    /// removing dim.
    #[arg(short, long, default_value_t = 7)]
    pub fade: u8,

    /// Bezier curve used for the animation
    #[arg(short, long, default_value = "default")]
    pub bezier: String,

    /// Strength of dim for windows that are the same class and floating
    ///
    /// Used for a permanent dim while working in dialog boxes such as file pickers.
    ///
    /// Note that the dim is removed when switching workspaces or doing any other event.
    #[arg(short = 'D', long, default_value_t = 0.7)]
    pub dialog_dim: f64,

    /// Show information about what hyprdim is doing
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
}
