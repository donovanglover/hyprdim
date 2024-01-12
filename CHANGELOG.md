# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

## 2.2.3 - 2024-01-12

This release prevents unknown events from being logged by hyprland-rs, useful for [hyprland-autoname-workspaces](https://github.com/hyprland-community/hyprland-autoname-workspaces) users.

### What's Changed

- chore: prevent log flood when some event are send by @cyrinux in https://github.com/donovanglover/hyprdim/pull/55
- chore(deps): Bump clap from 4.4.7 to 4.4.16 by @dependabot in https://github.com/donovanglover/hyprdim/pull/58
- chore(deps): Bump serde from 1.0.190 to 1.0.195 by @dependabot in https://github.com/donovanglover/hyprdim/pull/56
- chore(deps): Bump clap_complete from 4.4.4 to 4.4.6 by @dependabot in https://github.com/donovanglover/hyprdim/pull/53
- chore(deps): Bump ctrlc from 3.4.1 to 3.4.2 by @dependabot in https://github.com/donovanglover/hyprdim/pull/49
- chore(deps): Bump toml from 0.8.5 to 0.8.8 by @dependabot in https://github.com/donovanglover/hyprdim/pull/43

### New Contributors

- @cyrinux made their first contribution in https://github.com/donovanglover/hyprdim/pull/55

**Full Changelog**: https://github.com/donovanglover/hyprdim/compare/2.2.2...2.2.3

## 2.2.2 - 2023-10-29

- [feat(cli): Add colored output to --help](https://github.com/donovanglover/hyprdim/commit/c9be9b037616c5b929d177c8c5dfb82f34242d8d). Optionally disable with `NO_COLOR=1`.
- License changed to GPLv3+ since [hyprland-rs](https://github.com/hyprland-community/hyprland-rs) is GPL'd.

**Full Changelog**: https://github.com/donovanglover/hyprdim/compare/2.2.1...2.2.2

## 2.2.1 - 2023-08-13

This version fixes `dim_strength` not resetting when switching to a workspace with only one visible window with `--no-dim-when-only` enabled.

- fix: Reset dim_strength if no_dim_when_only by @donovanglover in https://github.com/donovanglover/hyprdim/pull/7

**Full Changelog**: https://github.com/donovanglover/hyprdim/compare/2.2.0...2.2.1

## 2.2.0 - 2023-08-13

This release features [a new `--dialog-dim` flag](https://github.com/donovanglover/hyprdim/blob/e8a10490b2b9ce584e4d31a0c2b873ce64e7ca32/src/cli.rs#L94-L106) that dims floating windows that have the same class. Additionally, [hyprdim can now be started with `hyprland.conf`](https://github.com/donovanglover/hyprdim/issues/4) once more.

- fix(is_special): Ensure that active window exists by @donovanglover in https://github.com/donovanglover/hyprdim/pull/5
- chore(deps): Bump serde from 1.0.182 to 1.0.183 by @dependabot in https://github.com/donovanglover/hyprdim/pull/1
- chore(deps): Bump clap from 4.3.19 to 4.3.21 by @dependabot in https://github.com/donovanglover/hyprdim/pull/3
- feat: Add --dialog-dim by @donovanglover in https://github.com/donovanglover/hyprdim/pull/6

**Full Changelog**: https://github.com/donovanglover/hyprdim/compare/2.1.0...2.2.0

## 2.1.0 - 2023-08-06

### Features

- [feat: Only allow one hyprdim instance](https://github.com/donovanglover/hyprdim/commit/3eef596d2c5a694aeb94c22ff3777ecfc5df322e)
- [feat: Add --verbose flag to control logging](https://github.com/donovanglover/hyprdim/commit/2ebd60675c3fb892b5859285e08a819a0942e5fa)
- [feat: Add --version flag](https://github.com/donovanglover/hyprdim/commit/77e1efcc9584e63b5bc098b5013d97f946492f47)
- [feat: Show project description when using --help](https://github.com/donovanglover/hyprdim/commit/0f8736eed79ef2f482d330a92c3a0117d2ba9d4a)
- [feat: Add author information to output](https://github.com/donovanglover/hyprdim/commit/35eb630e2a4914c4f07462d53f4a90dd42a5da87)
- [feat: Show long description in --help / man pages](https://github.com/donovanglover/hyprdim/commit/be398fc265f5f7c8aeafee4c6ed02002785ea949)
- [feat: Clarify fade animation speed range](https://github.com/donovanglover/hyprdim/commit/859b6ea46a329ba354007d8c4672e90386a4959c)
- [feat: Further limit fade animation speed range](https://github.com/donovanglover/hyprdim/commit/2ed56ba9104efa249fcd77adb1f7e239270cf308)
- [feat: Add --no-dim-when-only option](https://github.com/donovanglover/hyprdim/commit/07cbeee8d9bbb92926c743a7592b176408644e15)
- [feat: Add --ignore-entering-special option](https://github.com/donovanglover/hyprdim/commit/ce77e50c61b1b4598ed147c1f62d0ae4779b4223)
- [feat: Add --ignore-leaving-special option](https://github.com/donovanglover/hyprdim/commit/f01b80ec394f591c24cbb7fe8cf600eac4e63c5a)
- [feat: Don't dim initially if user doesn't want it](https://github.com/donovanglover/hyprdim/commit/3615b593a9d48dfc352ea8ba54abc166d726f922)
- [feat: Implement -n support for special workspaces](https://github.com/donovanglover/hyprdim/commit/1d40cb5e72482ab87a271afbaab7156181820fe3)
- [feat: Add PKGBUILD](https://github.com/donovanglover/hyprdim/commit/e5cfa1e7f756689e8746c3b414171eb307d7df8f)

### Documentation

- [docs: Add NixOS instructions to README](https://github.com/donovanglover/hyprdim/commit/a65db271bb0827e349e3ccb83b67ec9cae6a1a39)
- [cli: Clarify what duration waits for](https://github.com/donovanglover/hyprdim/commit/65e08b3fc58a14601c973d01af616907a1868777)
- [cli: Clarify that negative dim_strength is supported](https://github.com/donovanglover/hyprdim/commit/b43daee029fbff6d0d8f9b60e68bea97c19d9eea)
- [cli: Mention relation between fade speed and duration](https://github.com/donovanglover/hyprdim/commit/8f3916065fd999a67015eb7fea3cacf55c457c4d)
- [cli: Mention alternative to --persist](https://github.com/donovanglover/hyprdim/commit/d015c970c2eaf8472d4607539aab8665d171ab56)
- [docs: Add Arch Linux installation instructions](https://github.com/donovanglover/hyprdim/commit/a7a8974aa36b8e396c2a2bbe06d996f4a9d30876)
- [docs: Add guide for other distributions](https://github.com/donovanglover/hyprdim/commit/ad487a01dc5452554701f471eb602405a83a76d5)

**Full Changelog**: https://github.com/donovanglover/hyprdim/compare/2.0.1...2.1.0

## 2.0.1 - 2023-08-02

This version should function identically to 2.0.0, with the main difference being that unsafe blocks are no longer used.

**Full Changelog**: https://github.com/donovanglover/hyprdim/compare/2.0.0...2.0.1

## 2.0.0 - 2023-07-26

This release notably includes the breaking change of [renaming `hyprland-autodim` to `hyprdim`](https://github.com/donovanglover/hyprdim/commit/c8d0a13a603b671f3384ff27c6d25f7da63537ec). Besides that, [a `--persist` option was added](https://github.com/donovanglover/hyprdim/commit/0377381e18d1de97077529de71cafcf0fe2e8a98) to prevent changes to the hyprland variable `dim_inactive` from affecting hyprdim.

**Full Changelog**: https://github.com/donovanglover/hyprdim/compare/1.0.0...2.0.0

## 1.0.0 - 2023-07-25

- [feat: Restore original variables when stopping daemon](https://github.com/donovanglover/hyprland-autodim/commit/3094d4d54383393565a3830c1702423633a0c6ae)
- [feat: Add shell completions with clap_complete](https://github.com/donovanglover/hyprland-autodim/commit/2a52792901bf80e27c9bf6907a780613d08f78a1)
- [feat: Add man pages support with clap_mangen](https://github.com/donovanglover/hyprland-autodim/commit/c8a1329b4b7dafd0327084b3d17c73fc887822a6)
- [feat: Let users customize fade speed/bezier](https://github.com/donovanglover/hyprland-autodim/commit/6129190b8487f8e38e21739c6762298ca4a1e3d4)
- [feat: Let users customize strength/duration](https://github.com/donovanglover/hyprland-autodim/commit/d6d8490920460c89b11475adefdc8c357539d046)
- [feat: Keep track of initial variables](https://github.com/donovanglover/hyprland-autodim/commit/928aae77daf372357ad04d62a08a14b237365e9d)
- [feat: Enable dim_inactive on startup](https://github.com/donovanglover/hyprland-autodim/commit/d7e2c41b742299315955e119ef44df2c2a9b3bff)

**Full Changelog**: https://github.com/donovanglover/hyprland-autodim/compare/0.1.0...1.0.0

## 0.1.0 - 2023-07-25

- Initial release. Minimal viable product that works for my use case but lacks any sort of features.
