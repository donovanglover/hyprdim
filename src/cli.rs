use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    /// A value from 0 (no dim) to 1 (maximum dim)
    #[arg(short, long, default_value_t = 0.4)]
    pub strength: f64,

    /// How many milliseconds to wait
    #[arg(short, long, default_value_t = 800)]
    pub duration: u64,

    /// Fade animation speed
    #[arg(short, long, default_value_t = 7)]
    pub fade: u64,

    /// Bezier curve used for the animation
    #[arg(short, long, default_value = "default")]
    pub bezier: String,

    /// Prevent dim_inactive from being disabled by `hyprctl reload` etc
    #[arg(short, long, default_value_t = false)]
    pub persist: bool,

    /// Show information about what hyprdim is doing
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
}
