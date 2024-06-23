# racket-sys

[![crates.io](https://img.shields.io/crates/v/racket-sys.svg)](https://crates.io/crates/racket-sys)

Low level Racket language bindings for Rust

About Racket:
[https://racket-lang.org/](https://racket-lang.org/).

## Prerequisites:

- `Visual Studio` is installed
- Run `cargo` in `Developer PowerShell for VS`
- `Clang` toolchain is required to generate the bindings

## Environment Variables:

- `RACKET_CS_VERSION`: Only for MacOS. If not set, default to `8.13`.

- `RACKET_CS_HOME`: Racket installation directory. If not set, default to:
  - Windows: `C:\Program Files\Racket`
  - MacOS: `/Applications/Racket v{RACKET_CS_VERSION}`
  - Linux: TODO

## Examples:

- helloworld [examples/helloworld.rs](examples/helloworld.rs):

  ```
  cargo run --example helloworld
  ```

  screenshot on Windows:

  <img src="examples/helloworld.png" style="width:550px" />

Read [https://docs.racket-lang.org/inside/cs-embedding.html](https://docs.racket-lang.org/inside/cs-embedding.html) for more information on how to embed Racket in your application.

## TODO:

- [x] Windows support
- [x] MacOS support
- [ ] Linux support
