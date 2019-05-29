Testing Genesis
===

Right about meow, you may have noticed that the test setup can get quite tedious,
i.e. having to create new kitties for each unit test.

Wouldn't it be great to deploy our blockchain with some kitties already minted in the genesis block?

Substrate lets you deploy your chain with preconfigured storage through a genesis configuration.

In this section, we'll walk you through: 
- Extending `decl_storage` to add extra genesis data
- Mocking the genesis configuration in during testing
- Testing that we have the correct genesis set up

## Add Extra Genesis

Let's start by importing some functions and types we'll be using to configure genesis.

**substratekitties<span>.</span>rs**
```rust
use runtime_io::{with_storage, StorageOverlay, ChildrenStorageOverlay};
```
Notably:
- `with_storage`: is a function that takes a closure with global functions. This function lets you populate the runtime storage.
- `StorageOverlay`: is a set of key value pairs for storage. This represents where your storage resides.

Meow, let's configure our chain's genesis to accept some initial kitties, with predefined DNA & value.
And let's assign those kitties to designed owners.

Inside the `decl_storage` scope, create a struct called `add_extra_genesis` with the following implementation. 

```rust
decl_storage! {
    trait Store for Module<T: Trait> as KittyStorage { ... }

    add_extra_genesis {
        config(kitties): Vec<(T::AccountId, T::Hash, T::Balance)>;
        
        build(|storage: &mut StorageOverlay, _: &mut ChildrenStorageOverlay, config: &GenesisConfig<T>| {
            with_storage(storage, || {
                for &(ref acct, hash, balance) in &config.kitties {

                    let k = Kitty {
                                id: hash,
                                dna: hash,
                                price: balance,
                                gen: 0
                            };
                
                    let _ = <Module<T>>::mint(acct.clone(), hash, k);
                }
            });
        });
    }
}
```
The `config` attribute will expect a `kitties` config value, which will be a vector of tuples with types: 
- AccountId: the kitty owner
- Hash: the kitty id/dna
- Balance: the kitty's initial value

`build` subsequently uses the `kitties` configuration to build the storage itself.
In this implementation, it iterates over `kitties` and use the `mint` function to conveniently update
the rest of the storage items.

> **Note**: For this part to work, delete (or comment out) the deposit_event line in `mint()`, as `add_extra_genesis`
does not have context on how to handle events:
```
// Self::deposit_event(RawEvent::Created(to, kitty_id));
```

### Mock Genesis for Tests

Next, let's walk through setting the initial state for genesis in the testing environment.

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

### Test Genesis

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