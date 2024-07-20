mod bag;
mod bitvec;
mod encoded_data;
mod tree;

pub use bag::Bag;
use bitvec::BitVec;
pub use encoded_data::EncodedData;
use std::collections::HashMap;
use tree::Tree;

/// Encodes the given string following Huffman's
/// algorithm and returns it's encoded data.
pub fn encode(text: &str) -> EncodedData<char> {
    let tree = Tree::new(text.chars());
    let encoder = tree.encoder();

    let bits = text.chars().fold(BitVec::new(), |mut acc, x| {
        acc.extend(&encoder[&x]);
        acc
    });

    // Invert encoder to obtain decoder.
    let decoder: HashMap<_, _> = encoder
        .into_iter()
        .map(|(ch, encoding)| (encoding, ch))
        .collect();

    EncodedData::new(decoder, bits)
}

/// Decodes the encoded data and returns the original
/// text in the form of a `String`.
pub fn decode(encoded_data: EncodedData<char>) -> String {
    let (decoder, bits) = encoded_data.destructure();
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
