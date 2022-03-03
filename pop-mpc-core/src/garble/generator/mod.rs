pub mod half_gate;

use std::convert::TryInto;

use cipher::generic_array::GenericArray;
pub use half_gate::*;

use super::errors::GeneratorError;
use crate::garble::circuit::CompleteGarbledCircuit;
use crate::{circuit::Circuit, Block};
use cipher::{consts::U16, BlockCipher, BlockEncrypt};
use rand::{CryptoRng, Rng};

pub trait GarbledCircuitGenerator {
    /// Generates a garbled circuit
    fn garble<R: Rng + CryptoRng, C: BlockCipher<BlockSize = U16> + BlockEncrypt>(
        &self,
        c: &mut C,
        rng: &mut R,
        circ: &Circuit,
    ) -> Result<CompleteGarbledCircuit, GeneratorError>;
}

fn hash_parallel<C: BlockCipher<BlockSize = U16> + BlockEncrypt>(
    c: &mut C,
    blocks: &Vec<Block>,
    tweaks: &Vec<usize>,
) -> Vec<Block> {
    let tweaks: Vec<GenericArray<u8, U16>> = tweaks
        .into_iter()
        .map(|t| GenericArray::from((*t as u128).to_be_bytes()))
        .collect();

    let mut blocks_inner: Vec<GenericArray<u8, U16>> = blocks
        .into_iter()
        .map(|b| GenericArray::from(b.to_be_bytes()))
        .collect();
    c.encrypt_blocks(blocks_inner.as_mut_slice());

    let mut blocks = blocks_inner.clone();
    for (block, tweak) in blocks.iter_mut().zip(tweaks.iter()) {
        *block = block.iter().zip(tweak.iter()).map(|(a, b)| a ^ b).collect();
    }
    c.encrypt_blocks(blocks.as_mut_slice());

    let mut out_blocks: Vec<Block> = Vec::with_capacity(blocks.len());
    for (block, block_inner) in blocks.iter().zip(blocks_inner.iter()) {
        let block: GenericArray<u8, U16> = block
            .iter()
            .zip(block_inner.iter())
            .map(|(a, b)| a ^ b)
            .collect();
        let block_bytes: [u8; 16] = block
            .as_slice()
            .try_into()
            .expect("Expected array to have length 16");
        out_blocks.push(Block::from(block_bytes));
    }

    out_blocks
}
