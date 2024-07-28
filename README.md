# Huffman Coding

This is a simple implementation of a [Huffman Encoder/Decoder](https://en.wikipedia.org/wiki/Huffman_coding)

The program expects to receive input through `stdin` and returns output through `stdout`

## Use
```sh
# Use `-e` or `--encode` to encode data by bytes.
cat huffman_wiki.txt | cargo r -- -e > encoded.dat

# Use `-ew` or `--encode-words` to encode a string by words.
cat huffman_wiki.txt | cargo r -- -ew > encoded.dat

# Use `-d` or `--decode` to decode the data by bytes.
cargo r -- -d < encoded.dat

# Use `-dw` or `--decode-words` to decode the data to a string by words.
cargo r -- -dw < encoded.dat
```
