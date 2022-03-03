pub mod errors;
pub mod load;
pub mod parse;

use self::errors::CircuitEvalError;
use crate::gate::Gate;

pub use parse::*;

use std::collections::HashSet;

/// Circuit input
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CircuitInput {
    /// Circuit input id
    pub id: usize,
    /// Circuit input value
    pub value: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Circuit {
    /// Name of circuit
    pub name: String,
    /// Version of circuit
    pub version: String,
    /// Number of gates in the circuit
    pub ngates: usize,
    /// Number of wires in the circuit
    pub nwires: usize,
    /// Number of inputs to the circuit
    pub ninputs: usize,
    /// Number of wires for each input to the circuit
    pub input_nwires: Vec<usize>,
    /// Total number of input wires
    pub ninput_wires: usize,
    /// Total number of output wires
    pub noutput_wires: usize,
    /// All gates in the circuit
    pub(crate) gates: Vec<Gate>,
    /// Total number of AND gates
    pub nand: usize,
    /// Total number of XOR gates
    pub nxor: usize,
}

impl Circuit {
    pub fn new(
        name: String,
        version: String,
        ngates: usize,
        nwires: usize,
        ninputs: usize,
        input_nwires: Vec<usize>,
        ninput_wires: usize,
        noutput_wires: usize,
    ) -> Self {
        Circuit {
            name,
            version,
            ngates,
            nwires,
            ninputs,
            input_nwires,
            ninput_wires,
            noutput_wires,
            gates: Vec::with_capacity(ngates),
            nand: 0,
            nxor: 0,
        }
    }

    /// Evaluates the circuit in plaintext with the provided inputs
    pub fn eval(&self, inputs: Vec<CircuitInput>) -> Result<Vec<bool>, CircuitEvalError> {
        let mut wires: Vec<Option<bool>> = vec![None; self.nwires];
        for input in inputs.into_iter() {
            wires[input.id] = Some(input.value);
        }

        for (i, gate) in self.gates.iter().enumerate() {
            let (zref, val) = match *gate {
                Gate::Xor {
                    xref, yref, zref, ..
                } => {
                    let x =
                        wires[xref].ok_or_else(|| CircuitEvalError::UninitializedValue(xref))?;
                    let y =
                        wires[yref].ok_or_else(|| CircuitEvalError::UninitializedValue(yref))?;
                    (zref, x ^ y)
                }
                Gate::And {
                    xref, yref, zref, ..
                } => {
                    let x =
                        wires[xref].ok_or_else(|| CircuitEvalError::UninitializedValue(xref))?;
                    let y =
                        wires[yref].ok_or_else(|| CircuitEvalError::UninitializedValue(yref))?;
                    (zref, x & y)
                }
                Gate::Inv { xref, zref, .. } => {
                    let x =
                        wires[xref].ok_or_else(|| CircuitEvalError::UninitializedValue(xref))?;
                    (zref, !x)
                }
            };
            wires[zref] = Some(val);
        }

        let outputs = wires[(self.nwires - self.noutput_wires)..]
            .to_vec()
            .iter()
            .map(|w| w.unwrap())
            .collect();
        Ok(outputs)
    }
}

fn group_gates(mut gates: Vec<Gate>, ninput_wires: usize) -> Vec<Gate> {
    let mut grouped_gates: Vec<Gate> = vec![];
    let mut computed_inputs: HashSet<usize> = HashSet::new();
    for n in 0..ninput_wires {
        computed_inputs.insert(n);
    }
    let mut level_id = 0;
    while gates.len() > 0 {
        let mut level_outputs: Vec<usize> = vec![];
        let mut miss: Vec<bool> = vec![];
        for gate in gates.iter() {
            let mut hit = computed_inputs.contains(&gate.xref());
            if let Some(yref) = gate.yref() {
                hit &= computed_inputs.contains(&yref);
            }

            if hit {
                let mut gate = gate.clone();
                gate.set_level(level_id);
                level_outputs.push(gate.zref());
                grouped_gates.push(gate)
            }

            miss.push(!hit);
        }
        let mut miss_iter = miss.iter();
        gates.retain(|_| *miss_iter.next().unwrap());
        computed_inputs.extend(level_outputs);
        level_id += 1;
    }
    grouped_gates
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn test_gate_levels_are_independent() {
        let mut circ = Circuit::parse(
            "circuits/bristol/aes_128_reverse.txt",
            "aes_128_reverse",
            "",
        )
        .unwrap();
        for (level_id, level) in circ.gates.iter().group_by(|gate| gate.level()).into_iter() {
            let gates: Vec<Gate> = level
                .map(|a| *a)
                .filter(|gate| match gate {
                    Gate::And { .. } => true,
                    _ => false,
                })
                .collect();
            println!("{}", gates.len());
        }
    }

    // let mut outputs: HashSet<usize> = HashSet::new();
    //         for gate in group.iter().groupby(|| ) {
    //             match gate {
    //                 Gate::And {
    //                     xref, yref, zref, ..
    //                 } => {
    //                     if outputs.contains(&xref) {
    //                         panic!();
    //                     } else if outputs.contains(&yref) {
    //                         panic!();
    //                     } else {
    //                         outputs.insert(zref);
    //                     }
    //                 }
    //                 Gate::Xor {
    //                     xref, yref, zref, ..
    //                 } => {
    //                     if outputs.contains(&xref) {
    //                         panic!();
    //                     } else if outputs.contains(&yref) {
    //                         panic!();
    //                     } else {
    //                         outputs.insert(zref);
    //                     }
    //                 }
    //                 Gate::Inv { xref, zref, .. } => {
    //                     if outputs.contains(&xref) {
    //                         panic!();
    //                     } else {
    //                         outputs.insert(zref);
    //                     }
    //                 }
    //             }
    //         }
}
