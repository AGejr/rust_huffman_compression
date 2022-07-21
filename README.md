# rust_huffman_compression
A cli tool written in rust to compress files using [Huffman Compression](https://en.wikipedia.org/wiki/Huffman_coding#Compression)
```
usage: 
  rust_huffman_compression --interactive
  rust_huffman_compression -i <input filename> [optional args]
required
  --interactive         use cli input
  -i, --input           use file input
optional args:
  -o, --output          output filename
  -b, --binary          read input file as binary file
                        this option should be used when compressing large files
                        or when compressing other filetypes than text files
  -v, --verbose         increase verbosity
  -h, --help            print help message and exit
```
