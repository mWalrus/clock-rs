# Clock
A seven-segment clock with a TUI frontend using [cursive](https://github.com/gyscos/cursive).

![demo](./resources/demo.png)

:)

## Prerequisites
- rust & cargo
- cargo-make

## Installation
1. clone the repository and enter it: `git clone https://gitlab.com/mWalrus/clock-rs && cd clock-rs`
2. run cargo make: `cargo make install`

## Configuration
Configuration is made in the source since it doesn't really make sense to handle config files for such a small project.

In `src/clock.rs` you can find the variables `ON_COLOR` and `OFF_COLOR` at the top. These decide the color of the segments when they're on or off.
Change these to your liking and re-run `cargo make install` to apply your changes.