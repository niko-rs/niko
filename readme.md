# niko

🚧 extremely experimental / alpha 🚧

An XNA-like game engine for building games with wasm & webgl.

[demo on itch.io](https://vengarioth.itch.io/niko-test)

## getting started

niko uses [niko-cli](https://github.com/niko-rs/niko-cli) to help you get started quickly!

### niko setup

* First make sure you have cargo installed, or go to https://rustup.rs/.
* Then run `cargo install wasm-pack` because niko-cli will invoke it.
* After that run `cargo install niko-cli`
* Lastly, make sure `~/.cargo/bin` is in your `PATH`.

### creating a project

🚧 proof of concept 🚧

After you installed `wasm-pack` and `niko`, you can create a new niko project with `niko new <name>` (or `niko init`, just like cargo).

### running the development environment

🚧 proof of concept 🚧

When your project is set up, run `niko watch` in your project's directory. It builds your project and opens a browser window in your default browser with the result.
It will rebuild the project when a `.rs` file is changed in the current path.

## publishing to itch.io

🚧 not started 🚧

Run `niko bundle` in your project's directory and upload the `<project-name>.zip` to itch.io!

## license

[MIT](LICENSE)
