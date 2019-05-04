Common Patterns Moving Forward
===

## The Rust Compiler is Your Friend

One of the many advantages of using a strongly typed programming language like Rust is that the compiler can be very helpful and even suggest how to fix errors in your code. You can learn more about how the Rust compiler help you by reading [this post](https://jvns.ca/blog/2018/01/13/rust-in-2018--way-easier-to-use/). This will be obvious to you the most you write with Rust. To get started, you should check out the [Rust Book](https://doc.rust-lang.org/book/).

## Making Updates to Your Runtime

Before we jump into creating a custom Substrate runtime, you should be familiar with a few patterns which will help you iterate and run your code.

Your Substrate runtime code is compiled into two versions:

 - A [WebAssembly](https://webassembly.org/) (Wasm) image
 - A standard binary executable

The Wasm file is used as a part of the compilation of the standard binary, so it is important that you always compile your Wasm image first before you build the executable.

The pattern should be:

```bash
./scripts/build.sh               // Build Wasm
cargo build --release    // Build binary
```

Additionally, when you make changes to your node, the blocks produced in the past by older versions of your node persist. You may notice that when restarting your node, block production simply picks up where it left off.

However, if your changes to the runtime are significant, you may need to purge your chain of all the previous blocks with this handy command:

```bash
./target/release/substratekitties purge-chain --dev
```

After all this, then you will be able to start up your node again, fresh, with all the latest changes:

```bash
./target/release/substratekitties --dev
```

Remember this pattern; you will be using it a lot.

---

**Learn More**

You should always be using the latest version of Rust stable and nightly when hacking on Substrate.

We provide another script in the same directory as `build.sh` that you should run whenever you are starting a new project:

```bash
./init.sh
```

This script simply updates Rust, and ensures that you don't have strange compilation errors. If you remember, we already did this as a part of the workshop instructions.

---
