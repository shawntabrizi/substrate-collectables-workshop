Setting Up Tests
===

Our tests will require support code, so let's start by creating its own `test` module.
You can place this module inside `substratekitties.rs` as follows (alternatively, you can also refactor this out into its own `test.rs` file).

**substratekitties<span>.</span>rs**
```rust
// Your substratekitties code 

#[cfg(test)]
mod tests {
	// Your tests
}
```
The `#[cfg(test)]` attribute declares the entire `tests` module to be testing-only code.

Next, we import some test dependencies from external modules. Most of these modules are used to replace the config types of the traits that we want to implement in the test. 

A typical Substrate `test` uses the following external modules:

```rust
use super::*;
use support::{impl_outer_origin, assert_ok};
use runtime_io::{with_externalities, TestExternalities};
use primitives::{H256, Blake2Hasher};
use runtime_primitives::{
	BuildStorage, traits::{BlakeTwo256, IdentityLookup},
	testing::{Digest, DigestItem, Header}
}
```

Notably, `runtime_io` imports the following:

- [`TestExternalities`](https://crates.parity.io/sr_io/struct.TestExternalities.html) an in-memory, hashmap-based, externalities implementation. In other words, it mocks a test storage needed for the runtime to execute in a minimal fashion. `TestExternalities` accepts the generic type `Hasher`, hence we also import the `Blake2Hasher` to use it later on when building `TestExternalities`.

- [`with_externalities`](https://crates.parity.io/sr_io/fn.with_externalities.html) accepts two arguments, namely:
  - an object of type `Externalities`
  - a closure that is being executed given the first argument

From this, we can anticipate that the basic set up of each runtime test will be as follows:

```rust
with_externalities(some_externality, || {
	some_assertions!()
})
```

Don't forget, we also want to construct an Origin type for the test runtime. This step is usually called automatically by the `construct_runtime` macro. But during testing, we have do so manually.

```rust
impl_outer_origin! {
	pub enum Origin for KittiesTest {}
}
```

## Construct a Mock Runtime

Now we are ready to construct a mock runtime for our tests. 

First, we declare a main configuration type `KittiesTest`. `KittiesTest` will implement each of the configuration traits of modules used by the Kitties runtime, such as `system` and 	`balances`.

This mock runtime can be structured as follows:

```rust
#[derive(Clone, Eq, PartialEq)]
pub struct KittiesTest;

// Implement the system module traits
impl system::Trait for KittiesTest {}

// Implement the balances module traits
impl balances::Trait for KittiesTest {}

// Implement the trait for our own module, `super::Trait`
impl super::Trait for KittiesTest {}
```

We then implement the required traits for each module. For example, the `system` module requires the following traits implementations:

```rust
impl system::Trait for KittiesTest {
	type Origin = Origin;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type Digest = Digest;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = ();
	type Log = DigestItem;
}
```

> **Note:** In the test mock, we are able to simplify the value types passed into some of the traits. 
- `AccountId` can be represented by just a `u64` type
- `Event` is simply implemented with a unit `()`

## Create Test Externalities

At this point, we are ready to access and build the modules we just implemented traits for.

Let's assign a type alias to the Kitties module in order to easily access its methods going forward.

```rust
type Kitties = super::Module<KittiesTest>;
```

> **Note:** All modules are built in a way that the `Module` struct wraps all functions attached to it, hence the syntax `system::Module<KittiesTest>`.

Finally, we use a wrapper function to create the previously mentioned `TestExternalities`:

```rust
fn build_ext() -> TestExternalities<Blake2Hasher> {
	let mut t = system::GenesisConfig::<KittiesTest>::default().build_storage().unwrap().0;
	t.extend(balances::GenesisConfig::<KittiesTest>::default().build_storage().unwrap().0);
	// t.extend(GenesisConfig::<KittiesTest>::default().build_ext().unwrap().0);
	t.into()
}
```

This `build_ext` wrapper function will be subsequently used to construct mocks for each unit test. In most cases, it will simply build a genesis storage key/value store according to our desired mockup.

# Your Turn!

Set up the mock for your runtime test as specified above.

<!-- tabs:start -->

#### ** Template **

[embedded-code](./assets/5.1-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](./assets/5.1-finished-code.rs ':include :type=code embed-final')

<!-- tabs:end -->