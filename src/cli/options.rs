use clap::Parser;

use crate::cli::styles;
use crate::cli::LONG_ABOUT;

#[derive(Parser)]
#[command(author, version, about, long_about = LONG_ABOUT, styles = styles())]
pub struct Cli {
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

    /// Dim windows if they're the same class and floating (strength_default: 0.7)
    ///
    /// This option is particularly useful for dimming dialog boxes started by applications
    /// since those tend to have the same class and be floating.
    ///
    /// The dim is a permanent dim while working in the floating window of the same class.
    ///
    /// Note that the dim is removed when switching workspaces or doing any other event.
    ///
    /// Optionally specify a strength value to change how much dim is applied to dialog windows.
    /// The default strength value is 0.7.
    #[arg(short = 'D', long, value_name = "STRENGTH", default_value = None, default_missing_value = "0.7", num_args = 0..=1)]
    pub dialog_dim: Option<f64>,

    /// Show information about what hyprdim is doing
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
}
