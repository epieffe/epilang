# Epilang
Epilang is a high-level, interpreted, general-purpose programming language with a Rust-like syntax.

Visit the [Wiki](https://github.com/epieffe/epilang/wiki) for quickstart and documentation.

## Installation
Epilang standalone binaries can be downloaded from the [Release](https://github.com/epieffe/epilang/releases) page.

Pass a source file path as argument to the epilang executable to run it:
```bash
epilang path/to/file.epi
```
#### Interactive shell
Run the epilang executable with no args to start the Epilang interactive shell:
```bash
epilang
```
The following is an example interaction with the Epilang interactive shell:
```bash
epilang> let x = 3
epilang> x
3
epilang> x + 3
6
```

## Build from sources
To build this project from source you need [Rust](https://www.rust-lang.org/). To install it follow the instructions on the official [installation page](https://www.rust-lang.org/tools/install).

After installing you must be able to run `cargo` from the command line.

The following command prints the Cargo version, just to be sure it is installed.
```bash
cargo -V
```

Download Epilang source code from this repository and navigate to the source code directory on the command line:
```bash
git clone https://github.com/epieffe/epilang.git
cd epilang
```

#### Run from source
To create a debug build of the project and immediately start the Epilang interacive shell use the following command:
``` bash
cargo run
```

Or pass a file path as argument to run it:
``` bash
cargo run path/to/file.epi
```

#### Debug build
To create a debug build of the project use the following command:
```bash
cargo build
```
This command creates an executable file in `targert/debug/epilang`. Run this file to start Epilang.

#### Release build
To create an optimized build of the project use the following command:
```bash
cargo build --release
```
This command creates an executable file in `targert/release/epilang`. Run this file to start Epilang.
