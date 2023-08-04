# hyprdim

hyprdim is a daemon that automatically dims windows in [Hyprland](https://hyprland.org/) when switching between them.

## Features

- Easily see which window has focus, even with subtle or no borders.
- Windows only dim when switching windows, eliminating the need to toggle dim on/off when you want to see other windows.

## Installation

Add [`hyprdim`](https://search.nixos.org/packages?channel=unstable&query=hyprdim) to your `systemPackages` and rebuild.

```nix
{ pkgs, ... }:

{
  environment.systemPackages = with pkgs; [
    hyprdim
  ];
}
```

Alternatively, use `nix run nixpkgs#hyprdim` to test hyprdim without installing it.

## Usage

```fish
Usage: hyprdim [OPTIONS]

Options:
  -s, --strength <STRENGTH>  A value from 0 (no dim) to 1 (maximum dim) [default: 0.4]
  -d, --duration <DURATION>  How many milliseconds to wait [default: 800]
  -f, --fade <FADE>          Fade animation speed [default: 7]
  -b, --bezier <BEZIER>      Bezier curve used for the animation [default: default]
  -p, --persist              Prevent dim_inactive from being disabled by `hyprctl reload` etc
  -h, --help                 Print help
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
- [ ] Add to [awesome-hyprland](https://github.com/hyprland-community/awesome-hyprland)
- [ ] Add to [nixpkgs](https://github.com/NixOS/nixpkgs)

## Thanks

- [Yavor Kolev](https://github.com/yavko), [Cyril Levis](https://github.com/cyrinux), and [contributors](https://github.com/hyprland-community/hyprland-rs/graphs/contributors) for [hyprland-rs](https://github.com/hyprland-community/hyprland-rs)
- [Kevin K.](https://github.com/kbknapp), [Ed Page](https://github.com/epage), and [contributors](https://github.com/clap-rs/clap/graphs/contributors) for [clap-rs](https://github.com/clap-rs/clap)
- [Antti Keränen](https://github.com/Detegr) and [contributors](https://github.com/Detegr/rust-ctrlc/graphs/contributors) for [rust-ctrlc](https://github.com/Detegr/rust-ctrlc)
- [Liu BoFan](https://github.com/WLBF) and [contributors](https://github.com/WLBF/single-instance/graphs/contributors) for [single-instance](https://github.com/WLBF/single-instance)

[Arc]: https://doc.rust-lang.org/std/sync/struct.Arc.html
[Mutex]: https://doc.rust-lang.org/std/sync/struct.Mutex.html
