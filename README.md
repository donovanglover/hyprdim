# hyprdim

hyprdim is a daemon that automatically dims windows in [Hyprland](https://hyprland.org/) when switching between them.

## Features

- Easily see which window has focus, even with subtle or no borders.
- Windows only dim when switching windows, eliminating the need to toggle dim on/off when you want to see other windows.
- Prevent windows from being dimmed if there are no other visible windows in a workspace.
- Prevent windows from being dimmed when toggling special workspaces.
- Dim background windows when dialog windows open.

## Installation

### [NixOS](https://nixos.wiki/wiki/Overview_of_the_NixOS_Linux_distribution) (Recommended)

Add [`hyprdim`](https://search.nixos.org/packages?channel=unstable&query=hyprdim) to your `systemPackages` and rebuild.

```nix
{ pkgs, ... }:

{
  environment.systemPackages = with pkgs; [
    hyprdim
  ];
}
```

Alternatively, use `nix run nixpkgs#hyprdim` to start hyprdim without installing it.

### [Arch Linux](https://archlinux.org/)

I don't use Arch Linux anymore, but I wrote a PKGBUILD for the `pacman` enjoyers out there. Feel free to add it to the AUR.

```fish
git clone https://github.com/donovanglover/hyprdim -b 2.2.1 && cd hyprdim && makepkg -si
```

### Other distributions

Follow the [install guide](https://www.rust-lang.org/tools/install) for Rust. Then, use cargo to install hyprdim.

```fish
cargo install --git https://github.com/donovanglover/hyprdim --tag 2.2.1
```

Make sure `$HOME/.cargo/bin` is in your `$PATH` if it isn't already.

## Usage

```man
Usage: hyprdim [OPTIONS]

Options:
  -s, --strength <STRENGTH>      A value from 0 (no dim) to 1 (maximum dim) [default: 0.4]
  -d, --duration <DURATION>      How many milliseconds to wait before removing dim [default: 800]
  -f, --fade <FADE>              Fade animation speed from 0 (instantaneous) to 255 (very slow) [default: 7]
  -b, --bezier <BEZIER>          Bezier curve used for the animation [default: default]
  -p, --persist                  Prevent dim_inactive from being disabled by `hyprctl reload` etc
  -n, --no-dim-when-only         Don't dim when switching to a workspace that only has one visible window
  -i, --ignore-entering-special  Don't dim when opening a special workspace
  -I, --ignore-leaving-special   Don't dim when closing a special workspace
  -D, --dialog-dim [<STRENGTH>]  Dim windows if they're the same class and floating (strength_default: 0.7)
  -v, --verbose                  Show information about what hyprdim is doing
  -h, --help                     Print help (see more with '--help')
  -V, --version                  Print version
```

## Contributing

As far as I'm aware, this software is bug free. That said, if you know how to do things better, feel free to open an issue or make a pull request.

## Todo

- [x] Turn `dim_inactive` on if it isn't already
- [x] Restore the original state of variables when stopping the daemon
- [x] Add support for command line arguments
- [x] Add man pages
- [x] Add shell completions
- [x] Replace all unsafe code with [Arc][Arc], [Mutex][Mutex], etc.
- [ ] Add example image/video to README
- [x] Add to [awesome-hyprland](https://github.com/hyprland-community/awesome-hyprland)
- [x] Add to [nixpkgs](https://github.com/NixOS/nixpkgs)

## Thanks

- [Yavor Kolev](https://github.com/yavko), [Cyril Levis](https://github.com/cyrinux), and [contributors](https://github.com/hyprland-community/hyprland-rs/graphs/contributors) for [hyprland-rs](https://github.com/hyprland-community/hyprland-rs)
- [Kevin K.](https://github.com/kbknapp), [Ed Page](https://github.com/epage), and [contributors](https://github.com/clap-rs/clap/graphs/contributors) for [clap-rs](https://github.com/clap-rs/clap)
- [Antti Keränen](https://github.com/Detegr) and [contributors](https://github.com/Detegr/rust-ctrlc/graphs/contributors) for [rust-ctrlc](https://github.com/Detegr/rust-ctrlc)
- [Liu BoFan](https://github.com/WLBF) and [contributors](https://github.com/WLBF/single-instance/graphs/contributors) for [single-instance](https://github.com/WLBF/single-instance)

[Arc]: https://doc.rust-lang.org/std/sync/struct.Arc.html
[Mutex]: https://doc.rust-lang.org/std/sync/struct.Mutex.html
