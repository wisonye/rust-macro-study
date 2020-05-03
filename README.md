# Learn show to write customized `macro` in `Rust`

## Project structure

We need `workspace` in this study project. In the root folder, we got no source code expect a **`Cargo.toml`** file which represents the `workspace`.

And we got 2 `packages` inside the `workspace`:

- `my_macro_lib` - It's a `lib` which including the exported customized macro.

- `my_macro_consumer` - c

```rust
[workspace]

members = [
    "my_macro_lib",
    "my_macro_consumer",
]
```

</br>
<hr>

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

## How `macro` works:

Basically, if you write your own `macro`, you need to provide some sort of `macro patterns` which means If your macro get called, then then `matched patterns code` will `expand` to the source code position you called.

For example we got a customized macro `impl_login` below:

```rust
/// The macro which can implement the `Login` trait.
#[macro_export]
macro_rules! impl_login {

    //
    // Pass in a `Struct` type name, we implement the `Login` trait
    // for it automatic.
    //
    ($target_struct: ty) => {{
        impl Login for $target_struct {
            fn login(&self, user_name: &str, password: &str) -> bool {
                println!(
                    "login method get called with user_name: '{}' and password: '{}'",
                    user_name, password
                );
                true
            }
        }
    }};
}    
```    

When u call it like this:

```rust
struct UserService {}

impl_login!(UserService);
```

It actually will become this:

```rust
struct UserService {}

{
    impl Login for UserService {
        fn login(&self, user_name: &str, password: &str) -> bool {
            println!(
                "login method get called with user_name: '{}' and password: '{}'",
                user_name, password
            );
            true
        }
    }
}
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