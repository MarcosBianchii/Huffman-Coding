use huffman::{utils::SplitNonAlphabetic, EncodedData};
use std::{
    env,
    error::Error,
    io::{self, Read, Write},
};

fn main() -> Result<(), Box<dyn Error>> {
    let opt = env::args().nth(1).ok_or("No option was given")?;

    let huff: fn(Vec<u8>) -> Result<_, Box<dyn Error>> = match opt.as_ref() {
        "-e" | "--encode" => |input| {
            let encoded_data = huffman::encode(&input);
            Ok(encoded_data.into_bytes())
        },

        "-ew" | "--encode-words" => |input| {
            let text = String::from_utf8(input)?;
            let words: Vec<_> = text.split_non_alphabetic().collect();
            let encoded_data = huffman::encode(&words);
            Ok(encoded_data.into_bytes())
        },

        "-d" | "--decode" => |input| {
            let encoded_data = EncodedData::from_bytes(&input)?;
            Ok(huffman::decode(encoded_data))
        },

        "-dw" | "--decode-words" => |input| {
            let encoded_data = EncodedData::<String>::from_bytes(&input)?;
            Ok(huffman::decode(encoded_data).join("").into_bytes())
        },

        _ => Err("The given option is invalid")?,
    };

    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input)?;

    let bytes = huff(input)?;
    io::stdout().write_all(&bytes)?;

    Ok(())
}
