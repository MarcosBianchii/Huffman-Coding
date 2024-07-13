mod bag;
mod encoded_data;
mod tree;

use bitvec::prelude::*;
pub use encoded_data::EncodedData;
use tree::Tree;

pub fn encode(text: &str) -> EncodedData<char> {
    let tree = Tree::new(text.chars());
    let encoder = tree.encoder();

    let bits = text.chars().fold(BitVec::new(), |mut acc, x| {
        acc.extend_from_bitslice(&encoder[&x]);
        acc
    });

    EncodedData::new(tree, bits)
}

pub fn decode(encoded_data: EncodedData<char>) -> String {
    let (tree, bits) = encoded_data.destructure();
    let decoder = tree.decoder();

    let mut decoded_data = String::new();
    let mut acc = BitVec::new();
    for bit in bits {
        acc.push(bit);

        if let Some(&character) = decoder.get(&acc) {
            decoded_data.push(character);
            acc.clear();
        }
    }

    decoded_data
}
