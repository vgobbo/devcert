# DevCert

![rust.yml](https://github.com/vgobbo/devcert/actions/workflows/rust.yml/badge.svg)

Tool to easily generate development certificates (with CA).

This is totally based on [rcgen](https://crates.io/crates/rcgen), simply exposing an easy and somewhat flexible way of generating certificates.

## Usage

After installing (`cargo install --profile release devcert`), create a CA and a certificate:
```shell
devcert ca
devcert cert
```

The commands above will generate 4 files in the current directory, for the CA and the certificate.

If desired, it is also possible to re-use an existing PEM CA file:
```shell
devcert cert --ca myca
```

This expects files `myca.pem` and `myca.key` to exist in the current directory.

Specify `--help` to see more information about the tool sub-commands. Example:
```shell
devcert ca --help
```

## Developing

1. Install fish shell.
2. Install a recent (1.80+) rust compiler (with cargo).
3. Install a toolchain compatible with the desired target, like `stable-aarch64-apple-darwin`.
```shell
rustup toolchain install stable-aarch64-apple-darwin
```
4. Install a nightly profile compatible with the current machine, like:
```shell
rustup toolchain install nightly-aarch64-apple-darwin
```
5. Install rust packages:
```shell
cargo install --profile release taplo ripgrep
```
6. Setup hooks: `./scripts/setup-hooks`
7. (**Optional**) Run `./scripts/setup-target` to initialize `target/` in the temporary directory. This has to be done everytime the machine is restarted.

[ASN.1 Javascript Decoder](https://lapo.it/asn1js/) is useful for debugging generated certificates.