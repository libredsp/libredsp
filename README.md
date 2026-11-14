# LibreDSP Library

A Digital Signal Processing (DSP) library written in Rust. The library also contains WebAssembly (WASM) bindings for web applications.

LibreDSP provides functionality for:

1. Fast Fourier Transform (FFT)
2. Spectral Estimation
3. Frequency-Selective FIR and IIR Filter Design
4. Statistical Filter Design (e.g., Wiener Filter and LMS)
5. Resampling (via Decimation and Interpolation)
6. Speech Processing Algorithms, such as Background Noise Removal and Voice Activity Detection (VAD)
7. Direction-of-Arrival (DOA) Estimation

See the [Features](#features) section below for more information about the capabilities of this library.

The library has minimal dependencies and implements the core functionalities from scratch.

## Features
An overview of the features of the library is as follows:

### Frequency-selective Filter Design
- *FIR filters*: Window-based design (Kaiser, Hamming, Hann, and more)
- *Optimal FIR*: Least-squares and Parks-McClellan (Remez exchange) algorithms for linear-phase designs
- *IIR filters*: Analog-to-digital transformations (bilinear transform) and pole-zero placement

### Statistical Filter Design
- *Wiener filter*: Computes the filter coefficients that minimize the mean square error (MSE), an estimate of the expected value of the squared error between a desired signal and the filter output, in one computation.
- *Least mean square (LMS)*: Iteratively estimates the coefficients that minimize the MSE using instantaneous samples instead of estimating expected values by averaging.

### Spectral Estimation
Estimate the power spectral density (PSD) of your signal via:
- *Periodogram*: Quick FFT-based power spectrum
- *Welch's Method*: Smoother, lower-variance spectrum using overlapping windows

### Speech Processing
- *Background Noise Removal*: Background noise removal via Short-Time Fourier Transform (STFT).
- *Voice Activity Detection (VAD)*: Voice activity detection (VAD) based on logistic regression. This feature can detect speech and non-speech parts of an audio recording. Here, a non-speech part can be any type of sound, such as a car passing by or a honking sound, and is not necessarily silence.

### Direction-of-Arrival (DOA) Estimation
- *DOA* estimates the angle at which data is received on a an array of sensors.
- Currently, the library supports estimating DOA using the cross-correlation between the data received on two sensors.

## Installation
Simply clone the repo and build it with cargo. Alternatively, run:

```
cargo install libredsp
```

which adds the latest version of this library available on crates.io to your project.
For the web build (WASM bindings) run:

```
wasm-pack build --target web
```

## Examples
To run a specific example:

```
cargo run --example <EXAMPLE>
```

Where `<EXAMPLE>` for instance is `filter_design`. Visit the `examples/` directory to see all available examples.

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
- Simulates a system where the output of a sine-wave generator is added to the output of a white-noise generator, and the resulting signal is fed into a low-pass filter (LPF).
```rust
/* import statements */
fn main() {
    let mut graph = Graph::new();
    let mut display = Display::new();
    display.set_output_file("output.csv");

    /* Clean sine */
    let sine_id = graph.add_node(Generator::new(GeneratorType::Sine {
        n: 1000,
        amplitude: 1.0,
        frequency: 5.0,
        phase: 0.0,
    }));

    /* White noise */
    let noise_id = graph.add_node(Generator::new(GeneratorType::WhiteNoise {
        n: 1000,
        mean: 0.0,
        standard_deviation: 0.1,
    }));

    /* Add sine + noise */
    let mut signs = HashMap::new();
    signs.insert(sine_id, true);
    signs.insert(noise_id, true);
    let sum_id = graph.add_node(Sum::new(signs));

    /* Low-pass FIR */
    let filter_id = graph.add_node(Filter::new(TransferFunction {
        num: vec![
            0.0550, 0.0895, 0.1208, 0.1446, 0.1575, 0.1575, 0.1446, 0.1208, 0.0895, 0.0550,
        ],
        den: vec![1.0],
    }));

    let display_id = graph.add_node(display);

    /* Connect graph */
    graph.add_edge(sine_id, sum_id).unwrap();
    graph.add_edge(noise_id, sum_id).unwrap();
    graph.add_edge(sum_id, filter_id).unwrap();
    graph.add_edge(filter_id, display_id).unwrap();

    /* Run */
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

        /* We can see the non-zero peak at bin 5 */
        println!("{:?}", psd);
}
```

## License
Apache-2.0. See the LICENSE file for more info.
