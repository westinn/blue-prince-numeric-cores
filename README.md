# Blue Prince Numeric Core Solver

An over-engineered implementation of the Blue Prince Numeric Cores puzzles.

I implemented this as an exercise for myself as I went through ["The Rust Book"](https://doc.rust-lang.org/book/). I had already written this in Python 3, but wanted to try a refactor within Rust. Since this was my first Rust program, I wanted to try and implement as many Rust features, and utilize the most "Rusty" design patterns as possible; even if it bloats this puzzle solution.

I plan to have this accessible via WASM and Github Pages. The existing solutions are either all scripts or do not cover large inputs.

## Building

The Rust portion is it's own library, and used to take in a file as input. Since it now targets a basic web GUI, it now needs to be built using [`wasm-pack`](https://github.com/wasm-bindgen/wasm-pack):

```
wasm-pack build --target web
```

Or via the `build.sh` script provided.

## Developing

`cd` into the root of the project and run:

```
build.sh && python3 -m http.server
```

This will provide a localhost server at `http://[::1]:8000/`.
