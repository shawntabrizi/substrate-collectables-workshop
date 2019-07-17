Introduction
===

In this section, we will show you the basics of creating a custom runtime:

- How to use runtime storage
- How to expose runtime functions
- How to use the Polkadot-JS Apps UI to interact with your runtime

## What is a Runtime?

In short, the [*Runtime*](https://substrate.dev/docs/en/overview/glossary#runtime) is block execution logic of a blockchain, sometimes referred to as the state transition function [**STF**](https://substrate.dev/docs/en/overview/glossary#stf-state-transition-function-). In [Substrate](https://substrate.dev/docs/en/overview/glossary#substrate), this is stored on-chain in an implementation-neutral, machine-executable format as a WebAssembly binary. Other systems tend to express it only in human-readable format (e.g. Ethereum) or not at all (e.g. Bitcoin).

## What is a Module?

Your blockchain's runtime is composed of multiple features and functionalities which work together to power your blockchain. Things like:

- Account Management
- Token Balances
- Governance
- Runtime Upgrades
- and more...

These are all modules that are provided [here](https://github.com/paritytech/substrate/tree/master/srml) in the codebase for you to easily include into your runtime. These default set of modules provided by Substrate are known as the Substrate Runtime Module Library [**SRML**](https://substrate.dev/docs/en/overview/glossary#srml-substrate-runtime-module-library-)

With the Substrate framework, you are able to easily create and include new modules into your runtime. That is what we will be doing in this tutorial!

## Rust

At the moment, Substrate and Runtime development uses the [Rust programming language](https://www.parity.io/why-rust/).

This tutorial is **not** a course in learning Rust, but we should go over some of the basic differences you may encounter when following this guide compared to programming in other languages.

### Ownership and Borrowing

From the [Rust docs](https://doc.rust-lang.org/book/ownership.html):

> Ownership is Rust’s most unique feature, and it enables Rust to make memory safety guarantees without needing a garbage collector.
>
> - Each value in Rust has a variable that’s called its owner.
> - There can only be one owner at a time.
> - When the owner goes out of scope, the value will be dropped.

You will see throughout the tutorial we will add an ampersand (`&`) in front of some variables which implies that we are borrowing the value. This is useful if we need to reuse the value multiple times throughout a function.

It basically hinders you from doing stupid mistakes in handling memory - so be thankful if the Rust compiler advises you not to do a certain thing.

### Traits

From the [Rust docs](https://doc.rust-lang.org/book/traits.html):

> Traits abstract over behavior that types can have in common.

If you are familiar with interfaces, Traits are [Rust's sole notion of an interface](https://blog.rust-lang.org/2015/05/11/traits.html).

### Recoverable Errors with Result

You will learn later that module functions must return the `Result` type which allows us to handle errors within our functions. The returned `Result` is either `Ok()` for success or `Err()` for a failure. 

Throughout this tutorial we will use the question mark operator (`?`) at the end of functions which return a `Result`. When calling a function like this, for example `my_function()?`, Rust simply expands the code to:

```rust
match my_function() {
    Ok(value) => value,
    Err(msg) => return Err(msg),
}
```

You can learn more about [this here](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html).

Typical error of these kinds are

```
error[E0382]: borrow of moved value: `s`
error[E0382]: use of moved value: `s`
error[E0502]: cannot borrow `s` as immutable because it is also borrowed as mutable
error[E0499]: cannot borrow `s` as mutable more than once at a time
```

For example, the following code will not compile:

```rust
fn main() {
    let mut s = String::from("hello");
    let t1 = &s;      // t1 is an immutable reference to the String
    let t2 = &mut s;  // t2 is a mutable reference to the String
    t2.push_str(&t1); // We want to append t2 to t1.
                      //
		      // This is broken since both are references to the same underlying string.
}
```
Borrowing error:

```
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> src/main.rs:4:14
  |
3 |     let t1 = &s;
  |              -- immutable borrow occurs here
4 |     let t2 = &mut s;
  |              ^^^^^^ mutable borrow occurs here
5 |     t2.push_str(&t1);
  |                 --- immutable borrow later used here
```

An easy fix for this particular error is cloning the string instead of having another reference to it.

```rust
fn main() {
    let mut s = String::from("hello");
    let t1 = s.clone();
    let t2 = &mut s;
    t2.push_str(&t1);
}
```

Works!

### Macros

From the [Rust docs](https://doc.rust-lang.org/book/macros.html):

> While functions and types abstract over code, macros abstract at a syntactic level.

More simply, macros are code that write code, usually to simplify or make code more readable.

Substrate uses a lot of macros throughout the Runtime development process, and they are quite specific in the syntax they support and quite poor in the errors they return.

---
**Learn More**

 how does Result/? work maybe?

 Introduce Traits -> Connection to interfaces?

[TODO: make this a page]

---
