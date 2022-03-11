# Rustle

Like Wordle? Like using the terminal for everything? Rustle is a Wordle clone written for the terminal in Rust.

## Installation

Rustle requires [Rust](https://www.rust-lang.org/tools/install)

Build with either `cargo run` or `cargo build`. For extra fun, build it with the release profile (the `--release` flag).
I don't really think there's a visible improvement, but it sure is fun.

## Controls

Rustle has a pretty simple control scheme.

| Action | Key |
| ------ | --- |
| Quit | Esc |
| Quit | Ctrl + c |
| Delete Letter| Backspace |
| Delete Letter| Delete |
| Submit Guess | Enter |

Much like regular Wordle, you type in your answer and submit. Some terminals don't recognize Backspace, so the Delete
key is provided as an alternative.

## Contributions

Contributions are welcome. This is my first project written in Rust so there are many rough spots (be gentle!). If you
want to improve upon any of the content in this repository open a PR.
