# Substrate Collectables Workshop: Starting Template

This is the starting template for: https://github.com/shawntabrizi/substrate-collectables-workshop

## Setup

Follow [these installation instructions](https://docs.substrate.io/install/) to set up your development environment to work with the `polkadot-sdk`.

If you have already set up your computer for the `polkadot-sdk`, you should make sure your rust compiler is up to date with `rustup update`.

### test

To check that your code compiles successfully and is working correctly at each step, you can run:

```bash
cargo test
```

This executes the tests included in the `tests.rs` file.

You should run this now to make sure this starting template is compiling successfully for you.

As we add code to your project, we will also update the `tests.rs` file to include more tests for your Pallet.

At the beginning and end of every step, you should be able to run `cargo test` without warning or errors using the latest version of the `tests.rs` file.

If you have either, you should learn from them and fix them!

### rustfmt

To keep your code clean and easy to read, we use a tool called [`rustfmt`](https://github.com/rust-lang/rustfmt). To access all the latest features of `rustfmt` we specifically use the `nightly` toolchain.

To install `rustfmt` for `nightly`:

```bash
rustup component add rustfmt --toolchain nightly
```

To configure the behavior of `rustfmt`, we have included a `rustfmt.toml` file.

Try running:

```bash
cargo +nightly fmt
```

You shouldn't see any changes this time around, but as you write more code, you will be able to see `cargo +nightly fmt` make everything look pretty, consistent, and easy to read.

> We recommend you run `cargo +nightly fmt` after every step!

### clippy

[Clippy](https://github.com/rust-lang/rust-clippy) is a collection of lints to catch common mistakes and improve your Rust code. We also use the `nightly` toolchain here to gain access to the latest features.

To install `clippy` for `nightly`:

```bash
rustup component add clippy
```

Try running:

```bash
cargo +nightly clippy
```

Again, you shouldn't see any errors here, but as you write code for this tutorial, `clippy` can be used to help improve the quality of your code.

## Cheat Sheet

You should run these 3 commands at the end of every step without any errors or warnings.

```bash
cargo +nightly fmt
cargo +nightly clippy
cargo test
```
