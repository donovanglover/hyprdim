# hyprland-autodim

hyprland-autodim is a daemon that automatically dims windows when switching between them.

## Advantages

- Easy to see which window has focus, even with subtle or no borders.
- Windows only dim when switching windows, eliminating the need to toggle dim on/off when you want to see other windows.

## Usage

```fish
Usage: hyprland-autodim [OPTIONS]

Options:
  -s, --strength <STRENGTH>  A value from 0 (no dim) to 1 (maximum dim) [default: 0.4]
  -d, --duration <DURATION>  How many milliseconds to wait [default: 800]
  -f, --fade <FADE>          Fade animation speed [default: 7]
  -b, --bezier <BEZIER>      Bezier curve used for the animation [default: default]
  -h, --help                 Print help
```

## Contributing

As far as I'm aware, this software is bug free. Although it uses `unsafe`, the actual code is rather succinct and easy to verify.

That said, if you know how to do things better, feel free to open an issue or make a pull request. I'm particularly interested in alternatives that work without `unsafe`.

## Todo

- [x] Turn `dim_inactive` on if it isn't already
- [x] Restore the original state of variables when stopping the daemon
- [x] Add support for command line arguments
- [x] Add man pages
- [x] Add shell completions
- [ ] Add to [awesome-hyprland](https://github.com/hyprland-community/awesome-hyprland)
- [ ] Add to [nixpkgs](https://github.com/NixOS/nixpkgs)

## Thanks

- [Yavor Kolev](https://github.com/yavko), [Cyril Levis](https://github.com/cyrinux), and [contributors](https://github.com/hyprland-community/hyprland-rs/graphs/contributors) for [hyprland-rs](https://github.com/hyprland-community/hyprland-rs)
- [Kevin K.](https://github.com/kbknapp), [Ed Page](https://github.com/epage), and [contributors](https://github.com/clap-rs/clap/graphs/contributors) for [clap-rs](https://github.com/clap-rs/clap)
