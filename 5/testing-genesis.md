Testing Genesis
===

Right about meow, you may have noticed that the test setup can get quite tedious,
i.e. having to create new kitties for each unit test.

Wouldn't it be great to deploy our blockchain with some kitties already minted in the genesis block?

Substrate lets you deploy your chain with preconfigured storage through a genesis block configuration.

In this section, we'll walk you through: 
- Extending `decl_storage` to add extra genesis data
- Mocking the genesis configuration in during testing
- Testing that we have the correct genesis set up

## Various Approaches

There are 3 main approaches to configuring storage data in genesis.

- **Method 1**: You can hardcode storage items one by one, as you declare them in `decl_storage`. This is recommended when you need to configure simple values for storage items that are not so interdependent. You can see an example of how this is done [here](https://crates.parity.io/srml_support_procedural/macro.decl_storage.html#example). 

- **Method 2**: You can build storage items one by one, as you declare them in `decl_storage`. This is recommended when you need to build more complex storage items that are not so interdependent. You can see an example of how this is done [here](https://crates.parity.io/srml_support_procedural/macro.decl_storage.html#example).

- **Method 3**:  You can build the storage values in one go, after you declare them in `decl_storage`. This is recommended when you need to build complex storage items that may be interdependent. You can see an example of how this is done [here](https://crates.parity.io/srml_support_procedural/macro.decl_storage.html#genesisconfig).

In the kitties runtime, the storage values are highly interdependent, e.g. `KittyOwner` and `OwnedKittiesArray` store the same data. It would be rote to build each storage value one by one using methods 1, 2. 

Luckily, you have a `mint` function which modifies all the necessary storage items for you. Let's proceed with the 3rd method and use the `mint` function to build all the storage values just one time.

## Add Extra Genesis

For this exercise, let's configure our genesis block to contain 2 initial kitties, with predefined DNA & value. Let's also assign those kitties to designated owners.

Let's start by importing some functions and types you'll be using to configure genesis.

**substratekitties<span>.</span>rs**
```rust
use runtime_io::{with_storage, StorageOverlay, ChildrenStorageOverlay};
```
You'll shortly discover what these dependencies enable.

Next, inside the `decl_storage` scope, create a struct called `add_extra_genesis` with the following `config` value. 

```rust
decl_storage! {
    trait Store for Module<T: Trait> as KittyStorage { ... }

    add_extra_genesis {
        config(kitties): Vec<(T::AccountId, T::Hash, T::Balance)>;
    }
}
```

Put simply, `add_extra_genesis` allows you to add new fields into the genesis configuration. These new fields can be subsequently used to build additional storage or modify existing storage.

The new fields are named inside the `config()` attribute. In this case, we created an additional `kitties` genesis field. This `kitties` field expects a vector of tuples containing the following values: 
- AccountId: the kitty owner
- Hash: the kitty id/dna
- Balance: the kitty's initial value

Next, we need to initialize the storage items from this `kitties` config value using the existing `mint` function to build the storage values.

Inside `add_extra_genesis`, you can use a special `build` closure to execute the following logic:

```rust
build(|storage: &mut StorageOverlay, _: &mut ChildrenStorageOverlay, config: &GenesisConfig<T>| {
	with_storage(storage, || {
		for &(ref acct, hash, balance) in &config.kitties {

			let k = Kitty {	id: hash,
					dna: hash,
					price: balance,
					gen: 0
				};
		
			let _ = <Module<T>>::mint(acct.clone(), hash, k);
		}
	});
});
```

Inside `add_extra_genesis`, the `build` closure has access to all of the declared storage items, as well as the additional config values. This is why we are able to iterates over `kitties` and use the `mint` function to access and update all of the storage items at once.

Additionally, `StorageOverlay` and `childrenStorageOverlay` allows you to write into the runtime's storage tries. `with_storage` in this case is closure shorthand, which takes in the logic that will modify a particular storage space.

> **Note**: For this part to work, delete (or comment out) the deposit_event line in `mint()`, as `add_extra_genesis`
does not have context on how to handle events:
```
// Self::deposit_event(RawEvent::Created(to, kitty_id));
```

## Mock Genesis for Tests

Next, let's walk through actually passing in the initial genesis values in the testing environment.

Recall that you created an initial mock for tests using `TextExternalities`, where you simply
used default values for the initial chain state.

In the same function, you can now specify the initial `kitties` configuration. 
For simplicity, let's seed the chain with the following kitties: 
- 1 kitty with random DNA, belonging to account #0, worth 50 balance.
- 1 kitty with blank DNA, belonging to account #1, worth 100 balance.

```rust
fn build_ext() -> TestExternalities<Blake2Hasher> {
	let mut t = system::GenesisConfig::<KittiesTest>::default().build_storage().unwrap().0;
	t.extend(balances::GenesisConfig::<KittiesTest>::default().build_storage().unwrap().0);
	t.extend(GenesisConfig::<KittiesTest> {
		kitties: vec![  (0, H256::random(), 50), 
						(1, H256::zero(), 100)], 
	}.build_storage().unwrap().0);
	t.into()
}
```

## Test Genesis

If you set up your genesis configurations correctly, you should be able to successfully run
the following test: 
```rust
#[test]
fn should_build_genesis_kitties() {
	with_externalities(&mut build_ext(), || {
		// Check that 2nd kitty exists at genesis, with value 100
		let kitty0 = Kitties::kitty_by_index(0);
		let kitty1 = Kitties::kitty_by_index(1);

		// Check we have 2 kitties, as specified
		assert_eq!(Kitties::all_kitties_count(), 2);

		// Check that they are owned correctly
		assert_eq!(Kitties::owner_of(kitty0), Some(0));
		assert_eq!(Kitties::owner_of(kitty1), Some(1));

		// Check owners own the correct amount of kitties
		assert_eq!(Kitties::owned_kitty_count(0), 1);
		assert_eq!(Kitties::owned_kitty_count(2), 0);
	})
}
```

# Your Turn!

- Set up your genesis specs as specified above.
- Write some new tests to ensure that kitties are correctly configured at genesis.
- Refactor your previous tests to take advantage of this setup.
- Update your previous tests so they pass given the new genesis configs.

### Genesis Deployment
In the scope of this tutorial, we'll only review how to set up your genesis state
for testing. 

At this point, if you want to see how to configure the genesis block for an actual deployment, check out these
[chain specifications](https://github.com/paritytech/polkadot/blob/d102d8fbac950abf2a696097d65ec2edc64dc216/service/src/chain_spec.rs)
for our Alexander testnet.

<!-- tabs:start -->

#### ** Template **

[embedded-code](./assets/5.3-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](./assets/5.3-finished-code.rs ':include :type=code embed-final')

<!-- tabs:end -->