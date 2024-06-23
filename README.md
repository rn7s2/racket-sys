# racket-sys

[![crates.io](https://img.shields.io/crates/v/racket-sys.svg)](https://crates.io/crates/racket-sys)

Low level Racket language bindings for Rust

About Racket:
[https://racket-lang.org/](https://racket-lang.org/).

## Prerequisites:

- Base env:

  - `Rust` toolchain
  - `Clang` toolchain
  - `Racket` installation
  - Set environment variables (see below)

- Windows:

  - `Visual Studio` is installed

- MacOS:

  - `Xcode CommandLine tools` is installed
  - `iconv`, `ncurses` development libraries

- Linux:
  - Only tested on Ubuntu 24.04
  - Racket is installed through APT
  - `libncurses-dev`, `liblz4-dev`, `libzstd-dev` development libraries

## Environment Variables:

- `RACKET_CS_VERSION`: Only for MacOS. If not set, default to `8.13`.

- `RACKET_CS_HOME`: Racket installation directory. If not set, default to:
  - Windows: `C:\Program Files\Racket`
  - MacOS: `/Applications/Racket v{RACKET_CS_VERSION}`
  - Linux: `/usr`

## Examples:

- helloworld [examples/helloworld.rs](examples/helloworld.rs):

  ```
  cargo run --example helloworld
  ```

  screenshot on Windows:

  <img src="examples/helloworld.png" style="width:600px" />

- factorial [examples/factorial.rs](examples/factorial.rs) and [examples/factorial.rkt](examples/factorial.rkt):

  This example demos how to require Racket module from Rust code.

  ```
  cargo run --example factorial
  ```

  screenshot on Windows:

  <img src="examples/factorial.png" style="width:550px" />

  Note: set env variable `RKT_COLLECTS_DIR` to `racket installation/collects`. For example, on MacOS:
  ```
  RKT_COLLECTS_DIR=/Applications/Racket\ v8.13/collects cargo run --example factorial
  ```

Read [https://docs.racket-lang.org/inside/cs-embedding.html](https://docs.racket-lang.org/inside/cs-embedding.html) for more information on how to embed Racket in your application.

## TODO:

- [x] Windows support
- [x] MacOS support
- [x] Linux support
