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

Inside `decl_storage` scope, create a struct called `add_extra_genesis` with the following implementation. 

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
The `config` attribute declares a `kitty` configuration, which is a vector of tuples containing the following types: 
- AccountId: this will be the owner of the kitty
- Hash: This is the dna and id of the kitty
- Balance: This is the initial value of the kitty

This is how you will pass in your seed data, which we'll revisit shortly. 

The `build` attribute 
The `decl_storage` macro takes care of building the storage. 



Vec<(T::AccountId, T::Hash, T::Balance)>;

you can see an example of this in...
when you run the node, you'll do it up in chainspec. but we won't go into that much detail here

### testing kitties genesis

in our test, we can simulate deploying a genesis block which already gives some accounts kitties

test setup planned
account 1: has a kitty
account 2: has 2 kitties
accoutn 3: has 0 kitties

### configuring genesis in the node
<!-- address this later -->

# Your Turn!

Set up your genesis specs as specified above and get your tests to pass.

<!-- tabs:start -->

#### ** Template **

[embedded-code](./assets/5.1-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](./assets/5.1-finished-code.rs ':include :type=code embed-final')

<!-- tabs:end -->