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

    /// Prevent dim_inactive from being disabled by `hyprctl reload` etc
    ///
    /// Alternatively, set dim_inactive to true in hyprland.conf.
    #[arg(short, long, default_value_t = false)]
    pub persist: bool,

    /// Don't dim when switching to a workspace that only has one window
    ///
    /// Usually when you use dim in Hyprland, windows are dimmed even when switching between
    /// workspaces. This can be annoying if you constantly switch between two different workspaces
    /// that each consist of a single window.
    ///
    /// Note that this option also takes into account whether a window is fullscreen or not since
    /// a workspace with multiple windows but with an active one fullscreen makes dimming redundant.
    ///
    /// Note that this option also doesn't dim when switching between a special workspace that has
    /// only one window. In the future if Hyprland supports fullscreen windows inside special
    /// workspaces the codebase will need to be updated as appropriate. Please file an issue or
    /// make a pull request if this occurs and there hasn't been an update yet.
    #[arg(short, long, default_value_t = false)]
    pub no_dim_when_only: bool,

    /// Show information about what hyprdim is doing
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
}
