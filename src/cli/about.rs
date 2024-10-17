pub const ABOUT: &str = "
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
