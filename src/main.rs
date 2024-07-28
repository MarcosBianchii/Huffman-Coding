use huffman::{utils::SplitNonAlphabetic, EncodedData, HuffErr};
use std::{
    env,
    io::{self, Read, Write},
};

fn protocol() -> huffman::Result<()> {
    let opt = env::args().nth(1).ok_or(HuffErr::NoProtocolWasSpecified)?;

    let huff: fn(Vec<u8>) -> huffman::Result<Vec<u8>> = match opt.as_ref() {
        "-e" | "--encode" => |input| {
            let encoded_data = huffman::encode(&input)?;
            Ok(encoded_data.into_bytes())
        },

        "-ew" | "--encode-words" => |input| {
            let text = String::from_utf8(input)?;
            let words: Vec<_> = text.split_non_alphabetic().collect();
            let encoded_data = huffman::encode(&words)?;
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

        _ => Err(HuffErr::SpecifiedProtocolIsInvalid)?,
    };

    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input)?;

    let bytes = huff(input)?;
    io::stdout().write_all(&bytes)?;

    Ok(())
}

fn main() -> io::Result<()> {
    if let Err(err) = protocol() {
        let mut stderr = io::stderr();
        let msg = err.to_string();
        return stderr.write_all(msg.as_bytes());
    }

    Ok(())
}
