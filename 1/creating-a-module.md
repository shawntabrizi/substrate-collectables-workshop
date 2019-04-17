Creating a Module
===

To start, we need to create a new module for our runtime. For that we will work with an empty module template which we will place in a new `substratekitties.rs` file:

```
substratekitties
|
+-- runtime
    |
    +-- src
        |
        +-- lib.rs
        |
        +-- * substratekitties.rs
```

**substratekitties<span>.</span>rs**

```rust
use support::{decl_storage, decl_module};

pub trait Trait: system::Trait {}

decl_storage! {
    trait Store for Module<T: Trait> as KittyStorage {
        // Declare storage and getter functions here
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Declare public functions here
    }
}
```

You can see that this template allows us to start writing the most basic parts of our module, the public functions and the storage.

But before we even start doing that, we should include this file into our overall runtime which is defined in the `lib.rs` file located in the same directory.

## Updating our Runtime

If you take a closer look at the `lib.rs` file, you will notice it contains details about all the modules that make up your runtime. For each module, we:

- Import the Rust file containing the module
- Implement its `Trait`
- Include the module into the `construct_runtime!` macro

So we will need to do the same here.

To include the new module file we created, we can add the following line (indicated with the `// Add this line` comment) near the top of our file:

```rust
// `lib.rs`
...
pub type BlockNumber = u64;

pub type Nonce = u64;

// Add this line
mod substratekitties;
...
```

Since we have not defined anything in our module, our `Trait` implementation is also very simple. We can include this line after the other trait implementations:

```rust
// `lib.rs`
...
impl sudo::Trait for Runtime {
	type Event = Event;
	type Proposal = Call;
}

// Add this line
impl substratekitties::Trait for Runtime {}
...
```

Finally, we can add this line at the end of our `construct_runtime!` definition:

```rust
// `lib.rs`
...
construct_runtime!(
	pub enum Runtime with Log(InternalLog: DigestItem<Hash, Ed25519AuthorityId>) where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: system::{default, Log(ChangesTrieRoot)},
		Timestamp: timestamp::{Module, Call, Storage, Config<T>, Inherent},
		Consensus: consensus::{Module, Call, Storage, Config<T>, Log(AuthoritiesChange), Inherent},
		Aura: aura::{Module},
		Indices: indices,
		Balances: balances,
		Sudo: sudo,
		// Add this line
		Substratekitties: substratekitties::{Module, Call, Storage},
	}
);
...
```

Note that we have added three `types` to this definition (`Module`, `Call`, `Storage`), all of which are produced by the macros defined in our template.

As is, this code is valid and should compile. Give it a shot with:

```bash
./build.sh
cargo build --release
```

## Your Turn!

If you have not already, follow the instructions on this page to set up your `substrate-node-template`. If you completed everything successfully, you should be able to compile your code successfully without errors:

```bash
./build.sh
cargo build --release
```

At the end of every section in this tutorial, your code should compile without errors. Most of the changes throughout this tutorial will take place in the `substratekitties.rs` file, but we will need to update the `lib.rs` file one more time at a later point.

Now it's time to start adding some of our own logic!

<!-- tabs:start -->

#### ** Solution **

[embedded-code](./assets/1.1-finished-code.rs ':include :type=code embed-final')

<!-- tabs:end -->

---
**Learn More**

Check out the documentation for the `construct_runtime!` macro [here](https://docs.substrate.dev/docs/construct_runtime).

---
