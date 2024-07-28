mod encoded_data;
mod tree;
pub mod utils;

pub use encoded_data::EncodedData;
use std::{collections::HashMap, hash::Hash};
use tree::Tree;
use utils::{Bag, BitVec};

/// Encodes the given data following Huffman's
/// algorithm and returns it's encoded data.
pub fn encode<T: Hash + Ord>(data: &[T]) -> EncodedData<&T> {
    let tree = Tree::new(data);
    let encoder = tree.encoder();

    let bits = data.iter().fold(BitVec::new(), |mut acc, x| {
        acc.extend(&encoder[x]);
        acc
    });

    // Invert encoder to obtain decoder.
    let decoder: HashMap<BitVec, &T> = encoder
        .into_iter()
        .map(|(token, enc)| (enc, token))
        .collect();

    EncodedData::new(decoder, bits)
}

/// Decodes the encoded data and returns the original data.
pub fn decode<T: Clone>(encoded_data: EncodedData<T>) -> Vec<T> {
    let (decoder, bits) = encoded_data.destructure();
    let mut decoded_data = Vec::new();
    let mut acc = BitVec::new();

    for bit in bits {
        acc.push(bit);

        if let Some(token) = decoder.get(&acc) {
            decoded_data.push(token.clone());
            acc.clear();
        }
    }

    decoded_data
}
