use crate::{garble::circuit, Block};
use std::convert::TryInto;

include!(concat!(env!("OUT_DIR"), "/core.garble.rs"));

impl From<circuit::InputLabel> for InputLabel {
    #[inline]
    fn from(l: circuit::InputLabel) -> Self {
        Self {
            id: l.id as u32,
            label: l.label.into(),
        }
    }
}

impl From<InputLabel> for circuit::InputLabel {
    #[inline]
    fn from(l: InputLabel) -> Self {
        Self {
            id: l.id as usize,
            label: l.label.into(),
        }
    }
}

impl From<circuit::GarbledCircuit> for GarbledCircuit {
    #[inline]
    fn from(c: circuit::GarbledCircuit) -> Self {
        let mut table: Vec<u8> = Vec::with_capacity(c.table.len() * 32);
        for pair in c.table.into_iter() {
            table.append(&mut pair[0].to_be_bytes().to_vec());
            table.append(&mut pair[1].to_be_bytes().to_vec());
        }
        Self {
            generator_input_labels: c
                .generator_input_labels
                .into_iter()
                .map(|l| InputLabel::from(l))
                .collect(),
            table,
            public_labels: super::LabelPair {
                low: c.public_labels[0].into(),
                high: c.public_labels[1].into(),
            },
            output_bits: c.output_bits,
        }
    }
}

impl From<GarbledCircuit> for circuit::GarbledCircuit {
    #[inline]
    fn from(c: GarbledCircuit) -> Self {
        let mut table: Vec<[Block; 2]> = Vec::with_capacity(c.table.len() / 32);
        for pair in c.table.chunks_exact(32) {
            let low: [u8; 16] = pair[..16].try_into().unwrap();
            let high: [u8; 16] = pair[16..].try_into().unwrap();
            table.push([Block::from(low), Block::from(high)]);
        }
        Self {
            generator_input_labels: c
                .generator_input_labels
                .into_iter()
                .map(|label| circuit::InputLabel::from(label))
                .collect(),
            table,
            public_labels: [c.public_labels.low.into(), c.public_labels.high.into()],
            output_bits: c.output_bits,
        }
    }
}
