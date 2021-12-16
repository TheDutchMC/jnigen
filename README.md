# jnigen
Rust JNI wrapper generator

## Motivation
This program takes the output generated [siggen](https://github.com/TheDutchMC/siggen) and generated Rust code to call all methods in the file.

The end goal of both programs is to automatically generate Rust 'FFI' wrwapper for any Java library

## Installation
Requirments:
- Cargo toolchain
```
cargo install --git https://github.com/TheDutchMC/jnigen
```

## Usage
```
jnigen -i <input JSON> -o <Output directory>
```
The input JSON is the file produced by `siggen`. The output directory is the directory where `jnigen` will place the generated source code.

## License
`jnigen` is licensed under the MIT license