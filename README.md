# xmirror

[![Release](https://img.shields.io/github/v/release/foltik/xmirror?color=blue)](https://github.com/foltik/xmirror/releases)
[![Crates.io](https://img.shields.io/crates/v/xmirror.svg)](https://crates.io/crates/xmirror)
[![License: MIT](https://img.shields.io/badge/License-MIT-orange.svg)](https://opensource.org/licenses/MIT)

Capture keyboard and mouse events from a local machine and mirror them to a remote server over the network. I use it for remote controlling a media server with my laptop.

## Usage

* On $SERVER: `xmirror-server 0.0.0.0:1337`
* On another machine: `xmirror $SERVER:1337`

Hold `Ctrl + Shift + Alt + Super` simultaneously to exit.

## Installation

The supported platforms are:

* xmirror: MacOS
* xmirror-server: Linux+X11

### From a prebuilt binary

Download your binary of choice from the [releases page](https://github.com/foltik/xmirror/releases).

### From source (via crates.io)

```bash
cargo install xmirror
cargo install xmirror-server
```

## Development

This project uses Nix flakes for a dev environment with dependencies. With Nix installed, run `nix develop` to enter a dev shell with the right tools and dependencies for your platform.

If you have direnv installed, run `direnv allow .` to automatically start a dev shell when entering the project directory.
