# Blue Prince Numeric Core Solver

[Click here to use it!](https://westinn.github.io/blue-prince-numeric-cores/)

An over-engineered implementation of the Blue Prince Numeric Cores puzzles.

I implemented this as an exercise for myself as I went through ["The Rust Book"](https://doc.rust-lang.org/book/). I had already written this in Python 3, but wanted to try a refactor within Rust. Since this was my first Rust program, I wanted to try and implement as many Rust features, and utilize the most "Rusty" design patterns as possible; even if it bloats this puzzle solution.

I wanted a simpler solution than what I was finding, as the existing solutions are either terminal scripts or do not cover large inputs.

## Building

The Rust portion is it's own library and used to take in a file as input. Since it now targets a basic web GUI, it now needs to be built using [`wasm-pack`](https://github.com/wasm-bindgen/wasm-pack):

```bash
wasm-pack build --target web
```

Or via the `build.sh` script provided.

## Developing

`cd` into the root of the project and run:

```bash
build.sh && python3 -m http.server
```

This will provide a localhost server at `http://0.0.0.0:8000/`.

Or more simply, via the `run.sh` script provided.

## TODO

- ~~Update styling for GUI to match the game, Blue Prince (somewhat at least!).~~
- ~~Deploy to Github Pages.~~
- ~~Add a release pipeline that releases to Github Pages.~~
- Might clamp the possible return `NumericCore`s to be within 1-26. Currently, they are expanded past that. For the purposes of the actual game, no input should result in anything but a `NumericCoreValue` of 1-26.
- Include `tokei` in release pipeline to add code line counter to README. Just for fun!
- Add tests within Rust.
