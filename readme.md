# niko

An XNA-like game engine for building games with wasm & webgl.

## getting started

niko uses [niko-cli](https://github.com/niko-rs/niko-cli) to help you get started quickly!

### niko setup

First make sure you have cargo installed, or go to https://rustup.rs/. Then run `cargo install wasm-pack` and `cargo install niko-cli` (and make sure `~/.cargo/bin` is in your `PATH`).

### creating a project

After you installed niko and wasm-pack, you can create a new niko project with `niko new <name>` (or `niko init`, just like cargo).

### running the development environment

When your project is set up, run `niko watch` in your project's directory. It builds your project and opens a browser window in your default browser with the result.

## publishing to itch.io

Run `niko bundle` in your project's directory and upload the `<project-name>.zip` to itch.io!

## license

[MIT](LICENSE)
