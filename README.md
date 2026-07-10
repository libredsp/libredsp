# LibreDSP Library

A Digital Signal Processing (DSP) library written in Rust with WASM bindings for web applications.
The library provides functionality for spectral estimation, FIR and IIR 
filter design, hybrid simulation (such as the interconnection of discrete-time 
elements with a continuous plant), FFT computation, and other DSP-related 
operations.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
libredsp = "0.1.4"
```

or run:

```
cargo install libredsp
```

## Usage
To run a specific example:

```
cargo run --example <EXAMPLE>
```

Where `EXAMPLE` for instance is `filter_design`. Visit the `examples/` directory to see all avaiable examples.

## WebAssembly
To build for the web:

```
wasm-pack build --target web
```

### License
Apache-2.0. See the LICENSE file for more info.
