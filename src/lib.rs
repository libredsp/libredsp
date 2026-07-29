use wasm_bindgen::prelude::*;
pub mod fft;
pub mod filter_design;
pub mod math;
pub mod signal;
pub mod simulator;
pub mod spectral_estimation;
pub mod types;
pub mod utils;

use crate::filter_design::{
    fir_filter_least_squares_linear_phase, fir_filter_windowing, iir_filter_analog_to_digital,
    iir_filter_zero_pole_placement, parks_mcclellan_filter_design_by_nodes,
};
use crate::types::*;

use serde::Deserialize;
use std::collections::HashMap;

pub use crate::simulator::node_types::{DiscretePID, Display, Filter, Modifier, Plant, Generator, Sum};
pub use crate::simulator::{Graph, simulate};
pub use crate::types::*;

fn match_window_type(n: u8) -> WindowType {
    match n {
        0 => WindowType::Rectangular,
        1 => WindowType::Han,
        2 => WindowType::Hamming,
        3 => WindowType::Bartlett,
        // 4 => WindowType::Kaiser { min_stopband_attinuation: (0.0), transition_width: (0.0) },
        _ => panic!("Invalid window type"),
    }
}

fn match_filter_type(n: u8, w1: f64, w2: f64) -> FilterType {
    match n {
        0 => FilterType::Lowpass { w: w1 },
        1 => FilterType::Highpass { w: w1 },
        2 => FilterType::Bandpass { w1, w2 },
        3 => FilterType::Bandstop { w1, w2 },
        _ => panic!("Invalid filter type"),
    }
}

fn match_analog_to_digital_design_type(n: u8) -> AnalogToDigitalTransformationDesignMethod {
    match n {
        0 => AnalogToDigitalTransformationDesignMethod::Butterworth,
        1 => AnalogToDigitalTransformationDesignMethod::Chebyshev,
        _ => panic!("Invalid filter type"),
    }
}

fn match_linear_phase_filter_type(n: u8) -> LinearPhaseFilterType {
    match n {
        1 => LinearPhaseFilterType::I,
        2 => LinearPhaseFilterType::II,
        3 => LinearPhaseFilterType::III,
        4 => LinearPhaseFilterType::IV,
        _ => panic!("Invalid filter type."),
    }
}

#[wasm_bindgen]
pub fn windowing_method_wasm(
    n: usize,
    window_type_code: u8,
    filter_type_code: u8,
    w1: f64,
    w2: f64,
) -> TransferFunction {
    let filter_type = match_filter_type(filter_type_code, w1, w2);
    let window_type = match_window_type(window_type_code);
    fir_filter_windowing::windowing_method(n, window_type, filter_type)
}

// Receive flattened poles and zeros (in contrast to pairs) to make WASM happy.
#[wasm_bindgen]
pub fn zero_pole_placement_iir_filter_design_wasm(
    poles: Vec<f64>,
    zeros: Vec<f64>,
) -> TransferFunction {
    assert!(poles.len() % 2 == 0);
    assert!(zeros.len() % 2 == 0);

    let poles_pairs: Vec<(f64, f64)> = poles.chunks(2).map(|c| (c[0], c[1])).collect();
    let zeros_pairs: Vec<(f64, f64)> = zeros.chunks(2).map(|c| (c[0], c[1])).collect();

    iir_filter_zero_pole_placement::zero_pole_placement_iir_filter_design(
        &poles_pairs,
        &zeros_pairs,
    )
}

#[wasm_bindgen]
pub fn iir_filter_analog_to_digital_wasm(
    design_type_code: u8,
    filter_type_code: u8,
    w1: f64,
    w2: f64,
    n: usize,
    chebyshev_coef: f64,
) -> TransferFunction {
    let filter_type = match_filter_type(filter_type_code, w1, w2);
    let design_method = match_analog_to_digital_design_type(design_type_code);
    iir_filter_analog_to_digital::analog_to_digital_transform_iir_filter_design(
        design_method,
        filter_type,
        n,
        chebyshev_coef,
    )
}

#[wasm_bindgen]
pub fn least_squares_linear_phase_fir_wasm(
    f: Vec<f64>,
    a: Vec<f64>,
    weights: Vec<f64>,
    n: usize,
) -> TransferFunction {
    assert!(f.len() == a.len());
    fir_filter_least_squares_linear_phase::least_squares_linear_phase_fir(f, a, weights, n)
}

#[wasm_bindgen]
pub fn parks_mcclellan_wasm(
    f: Vec<f64>,
    a: Vec<f64>,
    weights: Vec<f64>,
    linear_phase_filter_type_code: u8,
    n: usize,
) -> TransferFunction {
    assert!(f.len() == a.len());
    assert!(f.len() == weights.len());

    let mut desired_freq_nodes: Vec<(f64, f64)> = Vec::new();
    let mut weights_nodes: Vec<(f64, f64)> = Vec::new();

    for i in 0..f.len() {
        desired_freq_nodes.push((f[i], a[i]));
        weights_nodes.push((f[i], weights[i]));
    }

    parks_mcclellan_filter_design_by_nodes(
        n,
        desired_freq_nodes,
        weights_nodes,
        match_linear_phase_filter_type(linear_phase_filter_type_code),
    )
}

//==============================================================================
//============================ SIMULATION FUNCTION =============================
//==============================================================================
/* Example json to de-serialize:
{
    "nodes": [
        {
            "id": "step",
            "type": "Step",
            "params": {
                "value": 2
            }
        },
        {
            "id": "pid",
            "type": "DiscretePID",
            "params": {
                "kp": 2,
                "ki": 1,
                "kd": 0.01,
                "dt": 0.01,
                "out_max": 1,
                "out_min": -1
            }
        },
        {
            "id": "plant",
            "type": "Plant",
            "params": {
                "transfer_function": {
                    "num": [
                        2,
                        5
                    ],
                    "den": [
                        1,
                        3,
                        2
                    ]
                },
                "sampling_period": 0.01,
                "dt": 0.01
            }
        },
        {
            "id": "disp",
            "type": "Display",
            "params": {}
        },
        {
            "id": "mod",
            "type": "Modifier",
            "params": {
                "mean": 0,
                "std_dev": 0.1
            }
        },
        {
            "id": "sum",
            "type": "Sum",
            "params": {
                "signs": {
                    "mod": "-",
                    "step": "+"
                }
            }
        }
    ],
    "edges": [
        {
            "from": "step",
            "to": "sum"
        },
        {
            "from": "sum",
            "to": "pid"
        },
        {
            "from": "pid",
            "to": "plant"
        },
        {
            "from": "plant",
            "to": "disp"
        },
        {
            "from": "plant",
            "to": "mod"
        },
        {
            "from": "mod",
            "to": "sum"
        }
    ],
    "simulation": {
        "steps": 100
    }
} */

#[derive(Deserialize)]
#[serde(tag = "type", content = "params")]
enum NodeSpec {
    Generator {
        #[serde(flatten)]
        generator_type: GeneratorType,
    },
    Sum {
        signs: HashMap<String, Sign>,
    },
    DiscretePID {
        kp: f64,
        ki: f64,
        kd: f64,
        dt: f64,
        out_max: f64,
        out_min: f64,
    },
    Plant {
        transfer_function: TransferFunction,
        sampling_period: f64,
        dt: f64,
    },
    Filter {
        transfer_function: TransferFunction,
    },
    Modifier {
        mean: f64,
        std_dev: f64,
    },
    Display {
        output_file: Option<String>,
    },
}

#[derive(Deserialize)]
struct NodeEntry {
    id: String,
    #[serde(flatten)]
    spec: NodeSpec,
}

#[derive(Deserialize)]
struct EdgeSpec {
    from: String,
    to: String,
}

#[derive(Deserialize)]
struct SimSpec {
    steps: usize,
}

#[derive(Deserialize)]
struct GraphSpec {
    nodes: Vec<NodeEntry>,
    edges: Vec<EdgeSpec>,
    simulation: SimSpec,
}

#[derive(Deserialize, Clone, Copy)]
enum Sign {
    #[serde(rename = "+")]
    Pos,
    #[serde(rename = "-")]
    Neg,
}

#[wasm_bindgen]
pub fn simulate_graph_wasm(json: &str) -> Result<JsValue, JsValue> {
    let spec: GraphSpec = serde_json::from_str(json)
        .map_err(|e| JsValue::from_str(&format!("JSON parse error: {e}")))?;

    let mut graph = Graph::new();
    let mut ids: HashMap<String, _> = HashMap::new();

    for entry in &spec.nodes {
        let node_id = match &entry.spec {
            NodeSpec::Generator { generator_type } => {
                graph.add_node(Generator::new(generator_type.clone()))
            }
            NodeSpec::DiscretePID {
                kp,
                ki,
                kd,
                dt,
                out_max,
                out_min,
            } => graph.add_node(DiscretePID::new(*kp, *ki, *kd, *dt, *out_max, *out_min)),

            NodeSpec::Plant {
                transfer_function,
                sampling_period,
                dt,
            } => graph.add_node(Plant::new(transfer_function.clone(), *sampling_period, *dt)),

            NodeSpec::Filter { transfer_function } => {
                graph.add_node(Filter::new(transfer_function.clone()))
            }

            NodeSpec::Modifier { mean, std_dev } => graph.add_node(Modifier::new(*mean, *std_dev)),

            NodeSpec::Display { output_file } => {
                let mut d = Display::new();
                if let Some(path) = output_file {
                    d.set_output_file(path);
                }
                graph.add_node(d)
            }

            NodeSpec::Sum { signs } => {
                let mut resolved = HashMap::new();
                for (ref_id, sign) in signs {
                    let nid = *ids.get(ref_id).ok_or_else(|| {
                        JsValue::from_str(&format!(
                            "Sum '{}' references '{}' before it's declared",
                            entry.id, ref_id
                        ))
                    })?;
                    resolved.insert(nid, matches!(sign, Sign::Pos));
                }
                graph.add_node(Sum::new(resolved))
            }
        };
        ids.insert(entry.id.clone(), node_id);
    }

    for e in &spec.edges {
        let from = *ids
            .get(&e.from)
            .ok_or_else(|| JsValue::from_str(&format!("unknown node id: {}", e.from)))?;
        let to = *ids
            .get(&e.to)
            .ok_or_else(|| JsValue::from_str(&format!("unknown node id: {}", e.to)))?;
        graph.add_edge(from, to).map_err(|err| {
            JsValue::from_str(&format!("edge error {} -> {}: {}", e.from, e.to, err))
        })?;
    }

    /* Return the simulation result captured by the Display elements. */
    let recordings = simulate(&mut graph, spec.simulation.steps)
        .ok_or_else(|| JsValue::from_str("cycle detected in graph"))?;

    let id_to_name: HashMap<usize, &String> = ids.iter().map(|(name, id)| (*id, name)).collect();

    let mut named_recordings: HashMap<&str, Vec<f64>> = HashMap::new();
    for (node_id, buf) in &recordings {
        if let Some(name) = id_to_name.get(node_id) {
            named_recordings.insert(name.as_str(), buf.clone());
        }
    }

    serde_wasm_bindgen::to_value(&named_recordings)
        .map_err(|e| JsValue::from_str(&format!("serialize error: {e}")))
}
