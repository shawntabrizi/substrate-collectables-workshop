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

Our `#[runtime::derive(RuntimeEvent)]` would aggregate these together, and allow you to access all possible events from a single object:

```rust
// Constructed by our
enum RuntimeEvent {
	Pallet1(Pallet1Event),
	Pallet2(Pallet2Event)
}
```

> NOTE: If you want to dive deeper into this, be sure to check out the [`rust-state-machine`](https://github.com/shawntabrizi/rust-state-machine) tutorial.

So at a high level, the runtime derive macros generate all of these aggregated types, which become available and can be used in our runtime and blockchain.

#### `#[runtime::pallet_index(n)]`

As we discussed earlier, FRAME's opinion on how to build a blockchain runtime is by allowing users to split up their state transition function into individual modules which we call pallets.

With the pallet index macro, you can literally see how we can compose a new runtime using a collection of pallets.

Our test runtime only needs three pallets to allow us to write good unit tests:

1. `frame_system`: This is required for any FRAME based runtime, so always included.
2. `pallet_balances`: This is a pallet which manages a blockchain's native currency, which will be used by our custom pallet.
3. `pallet_kitties`: This is the custom pallet we are building in this tutorial, and that we will test the functionality of.

You can see adding a pallet to your runtime is pretty simple:

```rust
#[runtime::pallet_index(2)]
pub type PalletKitties = pallet_kitties::Pallet<TestRuntime>;
```

Each pallet needs a unique index, and right now we only support 256 pallets maximum.

You can see that we extract the `Pallet` struct from each of the pallet crates. Since the `Pallet` struct is generic over `T: Config`, and because the `TestRuntime` struct will implement all the required traits, we can use it.

We can assign this to a type with any name we choose. To make things simple and explicit, we chose the name `PalletKitties`, which we can use to reference this specific pallet in all of our tests.

### Configuring your Runtime

Right below the `mod runtime` block, you will see us implement all the `Config` traits for our different pallets on the `TestRuntime` object.

```rust
impl pallet_kitties::Config for TestRuntime {
	type RuntimeEvent = RuntimeEvent;
}
```

This trait is named "Config", and you can really think about this as the configuration for each pallet in your runtime.

You can see `pallet_kitties` only exposes one configuration, which is basically asking us to pass back to it the `RuntimeEvent` generated by the `#[runtime::derive]` macro. If we didn't configure this right, our pallet would not be able to emit events (or even compile).

The `frame_system` and `pallet_balances` pallets also have a ton of configurations, but most of those are hidden and automatically configured thanks to the `config_preludes::TestDefaultConfig`:

```rust
#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for TestRuntime {
	type Block = Block;
	type AccountData = pallet_balances::AccountData<Balance>;
}
```

Configuring pallets is very specific to each pallet you add to your blockchain, and requires you to read the documentation of that pallet. These default configurations are suitable for more unit tests, but depending on your needs, you might want to change some of the configuration choices.

For the purposes of this tutorial, these `TestDefaultConfig` options are exactly what we need.

## Writing Tests

Now that we have constructed a test runtime with all the pallets we want to include, we can actually start writing unit tests.

### Test Externalities

Unit tests for the Polkadot SDK are just normal rust tests, but calling into our test runtime.

However, in a regular blockchain, you would have a database, and your pallet and pallet storage would call into this database and actually store the changes caused by your state transition function.

In our test environment, we must introduce a storage abstraction that will maintain state during a test and reset at the end.

For this, we create a new test externalities:

```rust
pub fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::<TestRuntime>::default()
		.build_storage()
		.unwrap()
		.into()
}
```

To use this text externalities, you need to execute your tests within a closure:

```rust
#[test]
fn my_pallet_test() {
	new_test_ext().execute_with(|| {
		// Your pallet test here.
	});
}
```

If you write a pallet test which uses some storage, and forget to wrap it inside the test externalities, you will get an error:

```rust
#[test]
fn forgot_new_test_ext() {
	System::set_block_number(1);
}
```

```text
---- tests::forgot_new_test_ext stdout ----
thread 'tests::forgot_new_test_ext' panicked at /Users/shawntabrizi/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sp-io-38.0.0/src/lib.rs:205:5:
`set_version_1` called outside of an Externalities-provided environment.
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

This error just tells you to add this code.

Don't forget to update your `tests.rs` file to include the test provided in this step.

It shows how you can:

- Set the blocknumber of your blockchain inside your tests.
- Call an extrinsic in your pallet from an `AccountId` of your choice.
- Check the extrinsic call completed `Ok(())`.
- Get the last event deposited into `System`.
- Check that last event matches the event you would expect from your pallet.

From this point forward, every step where you write some code will include new tests or modify existing tests.
Make sure to keep updating your `tests.rs` file throughout the tutorial.
