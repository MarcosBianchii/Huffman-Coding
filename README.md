# Huffman Encoding

This is a simple implementation of a [Huffman Encoder/Decoder](https://en.wikipedia.org/wiki/Huffman_coding)

The program expects to receive input through `stdin` and returns output through `stdout`

## Use
```sh
# Use `-e` or `--encode` to encode the given bytes
echo "text" | cargo run -- -e > example.dat

# Use `-d` or `--decode` to decode the given bytes
cargo run -- -d < example.dat
```
