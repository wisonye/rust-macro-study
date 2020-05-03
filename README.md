# Learn show to write customized `macro` in `Rust`

## How to run 

1. Run `npm install` for preparing the `hot-load` feature.

2. Run the scripts below to start live coding:

    ```bash
    # Hot-load for `my-macro-lib` package
    npm run start-lib

    # Hot-load for `my-macro-consumer` package
    npm run start-consumer
    ```
</br>
<hr>

## Add `cargo-expand` for doing a quick debug to our customized macro

1. Install

    ```bash
    # Install `cargo-expand`
    cargo install cargo-expand

    # Install the nightly version, as `cargo-expand` needed.
    rustup toolchain install nightly-x86_64-apple-darwin
    ```

2. How to run

    ```bash
    # `cargo expand` requires running against an actual package in this workspace
    cd my-macro-lib/

    # From now on, every time you want to check What content your marco expand (to real rust), just run:
    cargo expand

    # If you want to expand the test case inside your lib, then run :
    cargo expand --lib --tests
    ```