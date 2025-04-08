# Pallet Functions

As noted earlier, functions are easily implemented directly on top of the `Pallet` struct.

However, there are two types of functions exposed by Pallets:

1. Internal Functions
2. Callable Functions

Let's learn more about the differences.

## Internal Functions

Internal Pallet functions are just normal Rust functions. These are defined with regular Rust syntax and without any of the Pallet macros.

```rust
impl<T: Config> Pallet<T> {
	// None of the functions in this `impl` are callable by users.
}
```

They behave just like any Rust function from any Rust module. If they are marked `pub` they are exposed to other Rust modules, if not they are totally private.

What is most important is that no matter how these functions are marked, none of these functions are callable from outside of the blockchain, which means they are not exposed to users.

For that, we have Callable Functions.

## Callable Functions

If you are familiar with smart contracts or any kind of blockchain application, you would know that the way users interact with the blockchain is through transactions.

Those transactions are processed, and then dispatched to callable functions within the blockchain.

For ergonomics, you will normally see callable functions defined in the `lib.rs`, but their logic implemented in a separate `impls.rs` file. This is really up to your preference, but since this is the common practice in the Polkadot SDK, the tutorial will use this pattern as well.

### Pallet Call Macro

Pallet development allows you to create callable functions by introducing the `#[pallet::call]` macro on top of a normal function implementation code block.

```rust
#[pallet::call]
impl<T: Config> Pallet<T> {
	// All of the functions in this `impl` will be callable by users.
}
```

Unlike regular Rust functions, these callable functions have rules on how they must be defined.

### Origin

The first parameter of every callable function must be `origin: OriginFor<T>`.

The origin is an abstract concept for defining the where the call is coming from.

The most common origin is the signed origin, which is a regular transaction. We will learn about this more in the next section.

### Dispatch Result

Every callable function must return a `DispatchResult`, which is simply defined as:

```rust
pub type DispatchResult = Result<(), sp_runtime::DispatchError>;
```

So these functions can return `Ok(())` or some `Err(DispatchError)`.

You can easily define new `DispatchError` variants using the included `#[pallet::error]`, but we will get to that later.

## Terminology

In Polkadot, the term "call", "extrinsic", and "dispatchable" all get mixed together.

Here is a sentence which should help clarify their relationship, and why they are such similar terms:

> Users submit an extrinsic to the blockchain, which is dispatched to a Pallet call.

In this case, "extrinsic" is a more broad term than a "transaction".

An extrinsic is any message from the outside coming to the blockchain. A transaction is specifically a **signed** message coming from the outside.

In this tutorial, we will only deal with transactions, but again, learning more about extrinsics can be something to follow up on after you have learned the basics.
