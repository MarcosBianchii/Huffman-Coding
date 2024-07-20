# Huffman Encoding

This is a simple implementation of a [Huffman Encoder/Decoder](https://en.wikipedia.org/wiki/Huffman_coding)

The program expects to receive input through `stdin` and returns output through `stdout`

## Use
```sh
# Use `-e` or `--encode` to encode the given string
cat huffman_wiki.txt | cargo r -- -e > encoded.dat

# Use `-d` or `--decode` to decode the given bytes
cargo r -- -d < encoded.dat
```
