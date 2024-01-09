use clap::builder::styling::{AnsiColor, Effects, Styles};
use clap::Parser;

const LONG_ABOUT: &str = "
hyprdim is a daemon that automatically dims windows in Hyprland[1] when
switching between them. It works by setting the dim_inactive[2] variable
and changing dim_strength[2] based on whether windows should be dimmed
or not. This enables hyprdim to have smooth dim animations. Additionally,
hyprdim makes use of threads to keep track of new window events. This
enables hyprdim to only disable dim once a user has been inactive long
enough.

hyprdim is written in Safe Rust[3] and is available under the GPL license[4].
Anyone is free to study the software and expand upon it. The source code is
available here[5].

[1]: https://hyprland.org/

[2]: https://wiki.hyprland.org/Configuring/Variables/

[3]: https://doc.rust-lang.org/nomicon/meet-safe-and-unsafe.html

[4]: https://raw.githubusercontent.com/donovanglover/hyprdim/master/LICENSE

[5]: https://github.com/donovanglover/hyprdim

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.";

fn styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Red.on_default() | Effects::BOLD)
        .usage(AnsiColor::Red.on_default() | Effects::BOLD)
        .literal(AnsiColor::Blue.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::Green.on_default())
}

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

    /// Prevent dim_inactive from being disabled by `hyprctl reload` etc
    ///
    /// Alternatively, set dim_inactive to true in hyprland.conf.
    #[arg(short, long, default_value_t = false)]
    pub persist: bool,

    /// Don't dim when switching to a workspace that only has one visible window
    ///
    /// Usually when you use dim in Hyprland, windows are dimmed even when switching between
    /// workspaces. This can be annoying if you constantly switch between two different workspaces
    /// that each consist of a single window.
    ///
    /// Note that this option also takes into account whether a window is fullscreen or not since
    /// a workspace with multiple windows and an active one fullscreen makes dimming redundant.
    ///
    /// Note that this option also doesn't dim when switching between a special workspace that has
    /// only one window. In the future if Hyprland supports fullscreen windows inside special
    /// workspaces the codebase will need to be updated as appropriate. Please file an issue or
    /// make a pull request if this occurs and there hasn't been an update yet.
    #[arg(short, long, default_value_t = false)]
    pub no_dim_when_only: bool,

    /// Don't dim when opening a special workspace
    ///
    /// This is useful when you like toggling between a special workspace that has a transparent
    /// window and don't want to deal with a distracting dim in the background.
    ///
    /// Note that when you leave a special workspace, the workspace in the background will dim
    /// inactive windows if there's more than one window. If you want to avoid this, use this
    /// option in combination with ignore_leaving_special.
    ///
    /// Note that if you're watching a video you never want to dim, you can use windowrules in
    /// Hyprland to achieve this.
    #[arg(short, long, default_value_t = false)]
    pub ignore_entering_special: bool,

    /// Don't dim when closing a special workspace
    ///
    /// This is useful if you have multiple windows in your main workspace and usually don't switch
    /// between them.
    #[arg(short = 'I', long, default_value_t = false)]
    pub ignore_leaving_special: bool,

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

/// A helper function to only print what's happening to users if they enable the verbose flag.
pub fn log(text: &str) {
    let Cli { verbose, .. } = Cli::parse();

    if verbose {
        println!("{text}")
    }
}
