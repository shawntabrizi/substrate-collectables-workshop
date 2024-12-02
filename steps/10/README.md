# Runtime and Tests

The last thing we will cover in this section is a small dive into constructing a blockchain runtime and writing tests.

Our pallet could be dependant on many "external" factors:

- The current block number.
- Hooks when new blocks are built.
- Other pallets, for example to manage the blockchain balance.
- Specific configuration of your blockchain.
- etc...

All of these things are managed outside of our pallet, thus to write tests for a pallet, we must create a "test blockchain" where we would include and use the pallet.

A whole tutorial could be written just about configuring a runtime and writing tests, but that would be too much for this tutorial. Instead, we will just go over the basics, and hopefully in the near future, we can make time to write a dedicated tutorial for this and add a link to that here.

The content on this page is not super relevant for the tutorial, so if you choose to skip this section, or come back to it later, you won't miss much. But definitely you will want to come back to it later, as you can't become a good Polkadot SDK developer without writing tests!

## Constructing a Runtime

As we briefly covered earlier, the runtime is the state transition function for our blockchain.

In the context of writing unit tests for our pallet, we need not actually run a full, decentralized blockchain network, we just need to construct a runtime which places our custom pallet into our state transition function, and allows us to access it.

### `#[runtime]` Macro

The `#[runtime]` macro does all the work to build that state transition function that we can run tests on top of. You will see that much like our pallet macros, the runtime macros have an entry point:

```rust
#[runtime]
mod runtime {
	// -- snip --
}
```

You can see inside this entrypoint, we have various sub-macros:

- `#[runtime::runtime]`
- `#[runtime::derive]`
- `#[runtime::pallet_index(n)]`

While the `runtime` module does not look super big, you should know it generates a LOT of code. Much of it is totally hidden from you, since it is all dynamically generated boilerplate code to interface our runtime to the rest of our blockchain.

Let's look a little closer into these different sub-macros.

#### `#[runtime::runtime]`

Our whole blockchain runtime is represented by a single struct with the `#[runtime:runtime]` attribute:

```rust
#[runtime::runtime]
pub struct Runtime;
```

You can name this struct whatever you want. As you see in our tests, we name it `TestRuntime` for additional clarity. When you build a full Polkadot SDK project, you will probably have multiple runtimes, some for unit tests, some for test networks, and some for production. Because the Polkadot SDK is designed to be modular and configurable, it is super easy to do this, and construct many versions of your blockchain runtime

You can think of this runtime as just a placeholder for all of our runtime configuration and traits. The `TestRuntime` does not actually hold any data. It is a

More specifically, if you remember the `Config` trait that we must implement, `TestRuntime` will be the struct that implements all those traits and satisfies `Config`. We will see this below.

#### `#[runtime::derive]`

The runtime macros generate a lot of objects which give access to our state transition function and the pallets integrated in them.

You have already learned that pallets have:

- Callable Functions
- Events
- Errors
- etc...

The runtime macros generate "aggregated" runtime enums which represents all of those things across all pallets.

For example, imagine our blockchain has two pallets, each with one event. That would mean in our codebase, we would have two enums which look something like:

```rust
// Found in our Pallet 1 crate.
enum Pallet1Event {
	Hello,
}

// Found in our Pallet 2 crate.
enum Pallet2Event {
	World,
}
```

Our `#[runtime::derive(RuntimeEvent)]` would aggregate these together, and allow you to access all events from a single object:

```rust
// Constructed by our
enum RuntimeEvent {
	Pallet1(Pallet1Event),
	Pallet2(Pallet2Event)
}
```

#### `#[runtime::pallet_index(n)]`



## Writing Tests

Don't forget to update your `tests.rs` file to include the test provided in this step.

It shows how you can:

- Set the blocknumber of your blockchain inside your tests.
- Call an extrinsic in your pallet from an `AccountId` of your choice.
- Check the extrinsic call completed `Ok(())`.
- Get the last event deposited into `System`.
- Check that last event matches the event you would expect from your pallet.

From this point forward, every step where you write some code will include new tests or modify existing tests.
Make sure to keep updating your `tests.rs` file throughout the tutorial.
