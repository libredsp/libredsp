# LibreDSP Library

A Digital Signal Processing (DSP) library written in Rust with WASM bindings for web applications.
The library provides functionality for spectral estimation, FIR and IIR 
filter design, hybrid simulation i.e., the interconnection of discrete-time 
elements with continuous plants, FFT computation, and other DSP-related 
operations.

## Installation

Simply clone the repo and build it with cargo. Alternatively, run:

```
cargo install libredsp
```

To add the latest version of this library available on crates.io to your project.
For the web build (WASM bindings) run:

```
wasm-pack build --target web
```

## Usage
To run a specific example:

```
cargo run --example <EXAMPLE>
```

Where `<EXAMPLE>` for instance is `filter_design`. Visit the `examples/` directory to see all avaiable examples.

---
## Features

An overview of the features of the library is as follows:

### Filter Design
- **FIR filters**: Window-based design (Kaiser, Hamming, Hann, and more)
- **Optimal FIR**: Least-squares and Parks–McClellan (Remez exchange) algorithms for linear-phase designs
- **IIR filters**: Analog-to-digital transformations (bilinear transform, impulse invariance) and pole-zero placement

### Spectral Estimation
Understand the frequency content of your signals:
- *Periodogram*: Quick FFT-based power spectrum
- *Welch's Method*: Smoother, lower-variance spectrum using overlapping windows

### Hybrid Simulation
Model and simulate systems where *discrete-time* elements interact with *analog plants*:
- Discrete-time PID, filters, and custom transfer functions
- Continuous-time plant models (via ODE solvers or state-space)

---

## Usage

- Filter design example:
```rust
/* import statements */
fn main() {
    /* Windowing method */
    let coefs = windowing_method(6, WindowType::Rectangular, FilterType::Lowpass{w: 0.5});
    /* Analog-to-digital transform method */
    let coefs_iir_ad: TransferFunction = analog_to_digital_transform_iir_filter_design(
            AnalogToDigitalTransformationDesignMethod::Butterworth, 
            FilterType::Highpass { w: 0.5 }, 
            3,
            0.2);
    /* Linear phase least-squares */
    let fir_ls_lp = least_squares_linear_phase_fir(
        vec![0.0, 0.15, 0.85, 1.0], 
        vec![1.0, 1.0, 0.0, 0.0],
        vec![1.0, 100.0],
        11);

     /* Parks-Mcclellan example */
    let num_taps = 11;
    let desired_freq = |f: f64| {
        if f <= 0.18 * std::f64::consts::PI {
            1.0
        } else {
            0.0
        }
    };
    let weight = |f: f64| {
        if f <= 0.18 * std::f64::consts::PI {
            1.0
        } else {
            10.0
        }
    };
    let coefficients = parks_mcclellan(num_taps, desired_freq, weight, LinearPhaseFilterType::I);
}
```
- Simulating a system consisting of a discrete-PID controller connected to a continous plant via feedback:

```rust
/* import statements ... */
fn main() {
    let mut graph = Graph::new();
    let mut display = Display::new();
    display.set_output_file("output.csv");

    /* Add nodes */
    let step_id = graph.add_node(Step::new(2.0));
    let display_id = graph.add_node(display);
    let pid_id = graph.add_node(DiscretePID::new(1.0, 10.0, 0.01, 0.01, 1.0, -1.0));
    let plant_id = graph.add_node(Plant::new(
        TransferFunction { num: vec![2.0, 5.0], den: vec![1.0, 3.0, 2.0] },
        0.01,
        0.001
    ));
    
    let modifier_id = graph.add_node(Modifier::new(0.0, 0.2));

    /* Configuring the nodes */
    let mut signs = HashMap::new();
    signs.insert(step_id, true);       // Step: positive (+)
    signs.insert(modifier_id, false);  // Feedback: negative (-)
    let sum_id = graph.add_node(Sum::new(signs));

    
    graph.add_edge(step_id, sum_id).unwrap();
    graph.add_edge(sum_id, pid_id).unwrap();
    graph.add_edge(pid_id, plant_id).unwrap();
    graph.add_edge(plant_id, display_id).unwrap();
    graph.add_edge(plant_id, modifier_id).unwrap();
    graph.add_edge(modifier_id, sum_id).unwrap();

    simulate(&mut graph, 1000);
}
```

- Spectral estimation example:
```rust
/* import statements */
fn main() {
        let n = 64;
        let freq = 5.0;
        let amplitude = 1.0;
        
        let signal = get_sine_signal(n, amplitude, freq, 0.0) 
                                + get_white_noise_signal(n,2.0,0.0);
        let psd = signal.welch_estimate(libredsp::types::WindowType::Rectangular, 64, 5);

        // We can see the non-zero peak at bin 5
        println!("{:?}", psd);
}
```

## License
Apache-2.0. See the LICENSE file for more info.
