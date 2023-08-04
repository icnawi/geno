# Geno CLI (WIP)

The tool to generate pseudo random strings of ASCII characters as passwords, secrets or codes.
Currently this tool supports 3 formats of output:
- Numeric
- HEX
- Alphanumeric

## Installation

To install locally, clone repo and in the folder of the project build the binary:
```
$ cargo build --release
```
After this it's ready to use:
```
$ ./target/release/geno --algorithm <algorithm> --length <length> --number <number> --output <output>

```

WIP: Later will be added Shell scripts to be able add it to your `$PATH`

## Usage 

```
USAGE:
    geno [OPTIONS]
    geno [FLAGS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -A, --algorithm <algorithm>    Generation algorithm [default: alphanumeric]
    -L, --length <length>          Length of the generated code [default: 6]
    -N, --number <number>          Number of codes to generate [default: 1]
    -O, --output <output>          Output file path [default: dist/codes.json]

```

Basic example of how to generate 1000 strings of length 8 in numeric format:
```
$ geno -N 1000 -L 8 -O nums.json
```

The output (the default algorithm is Alphanumeric):
```json
{
  "codes": [
    "tl7nwt9i",
    "dkj9bvj1",
    "gbuzfjbl",
    "sg2p24yk",
    ...
  ]
}
```

Another example with `-A` or `--algorithm` option. The example below shows how to generate 25000 CSS codes:
```
$ geno -A hex -N 25000 -O css_codes.json
```

Output:
```json
{
  "codes": [
    "4ca4dd",
    "792319",
    "b4b267",
    "8a47bb"
    ...
  ]
}
```
