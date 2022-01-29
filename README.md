# tui-util

## !!UNDER CONSTRUCTION!!
This project is under construction, and will likely have interface changes

## About
tui-util is a minimal tool creating interactive tui software. I built tui-util as
a learning exercise, to practice using rust and better understand how terminal
programs work under the hood.

tui-util is inspired by tools like [crossterm](https://github.com/crossterm-rs/crossterm)
, [termion](https://github.com/redox-os/termion), and [ncurses](https://invisible-island.net/ncurses/announce.html).

## Roadmap
* [x] an interface for setting the tty into raw mode
* [ ] an interface for writing [ANSI CSI and SGR codes](https://en.wikipedia.org/wiki/ANSI_escape_code) to the tty
* [ ] an event handling system for reacting to keyboard, mouse, and window resize events

## Running the examples
1. clone this repo
2. `cargo run --example basic`

## NON-Goals
* windows support
* UI elements
