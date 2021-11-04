# oeis-min

This is a Rust tool that looks for the smallest number that is not in the [The On-Line Encyclopedia of Integer Sequences (OEIS)](https://oeis.org), in an attempt to answer the [Interesting number paradox](https://en.wikipedia.org/wiki/Interesting_number_paradox).

Most of the work for this project is implemented in [oeis-utils](https://github.com/jakevossen5/oeis-utils) crate.

## Usage

First, download and extract a copy of the OEIS database from [https://oeis.org/stripped.gz](https://oeis.org/stripped.gz)

```bash

git clone git@github.com:jakevossen5/oeis-min.git
cd oeis-min
cargo run --release -- path/to/stripped
```

You can also check the work with the bash script by running `./oeis-min.sh /path/to/stripped`.
This depends on [ripgrep](https://github.com/BurntSushi/ripgrep) being in your `$PATH`
