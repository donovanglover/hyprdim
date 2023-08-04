use clap::Parser;

const LONG_ABOUT: &str = "
hyprdim is a daemon that automatically dims windows in Hyprland[1] when
switching between them. It works by setting the dim_inactive[2] variable
and changing dim_strength[2] based on whether windows should be dimmed
or not. This enables hyprdim to have smooth dim animations. Additionally,
hyprdim makes use of threads to keep track of new window events. This
enables hyprdim to only disable dim once a user has been inactive long
enough.

hyprdim is written in Safe Rust[3] and is available under the MIT license[4].
Anyone is free to study the software and expand upon it. The source code is
available here[5].

[1]: https://hyprland.org/

[2]: https://wiki.hyprland.org/Configuring/Variables/

[3]: https://doc.rust-lang.org/nomicon/meet-safe-and-unsafe.html

[4]: https://raw.githubusercontent.com/donovanglover/hyprdim/master/LICENSE

[5]: https://github.com/donovanglover/hyprdim
";

#[derive(Parser)]
#[command(author, version, about, long_about = LONG_ABOUT)]
pub struct Cli {
    /// A value from 0 (no dim) to 1 (maximum dim)
    #[arg(short, long, default_value_t = 0.4)]
    pub strength: f64,

    /// How many milliseconds to wait
    #[arg(short, long, default_value_t = 800)]
    pub duration: u64,

    /// Fade animation speed from 0 (instantaneous) to 65535 (very slow)
    #[arg(short, long, default_value_t = 7)]
    pub fade: u16,

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
