use huffman::EncodedData;
use std::{
    env,
    error::Error,
    io::{self, Read, Write},
};

fn main() -> Result<(), Box<dyn Error>> {
    let opt = env::args().nth(1).ok_or("No option was given")?;

    let data = match opt.as_ref() {
        "-e" | "--encode" => {
            let mut input = String::new();
            io::stdin().read_to_string(&mut input)?;
            let encoded_data = huffman::encode(&input);
            encoded_data.into_bytes()
        }

        "-d" | "--decode" => {
            let mut input = Vec::new();
            io::stdin().read_to_end(&mut input)?;
            let encoded_data = EncodedData::from_bytes(&input)?;
            huffman::decode(encoded_data).into_bytes()
        }

        _ => Err("The given option is invalid")?,
    };

    io::stdout().write_all(&data)?;
    Ok(())
}
