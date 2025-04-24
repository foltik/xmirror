# xmirror

[![Release](https://img.shields.io/github/v/release/foltik/xmirror?color=blue)](https://github.com/foltik/xmirror/releases)
[![Crates.io](https://img.shields.io/crates/v/xmirror.svg)](https://crates.io/crates/xmirror)
[![License: MIT](https://img.shields.io/badge/License-MIT-orange.svg)](https://opensource.org/licenses/MIT)

Capture keyboard and mouse events from a local machine and mirror them to a remote server over the network. I use it for remote controlling a media server with my laptop.

## Usage

* On $SERVER: `xmirror-server 0.0.0.0:1337`
* On another machine: `xmirror $SERVER:1337`

Hold `Ctrl + Shift + Alt + Super` simultaneously to exit.

## Platform Support

* xmirror: MacOS
* xmirror-server: Linux+X11
* PRs welcome!

## Installation

### From a prebuilt binary

Download your binary of choice from the [releases page](https://github.com/foltik/xmirror/releases).

### From source (via crates.io)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install xmirror
cargo install xmirror-server
```
