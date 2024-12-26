# DevCert

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