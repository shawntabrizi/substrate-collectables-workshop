Storage Mapping
===

Unfortunately, our last runtime only allowed us to store a single value across all users of our blockchain. As we build toward our collectables chain, it will make sense for each user to own their own values.

For this we can replace our simple single value storage with a storage mapping:

```
use system::ensure_signed;
use srml_support::{StorageMap, dispatch::Result};

pub trait Trait: balances::Trait {}

decl_storage! {
    trait Store for Module<T: Trait> as CryptokittiesStorage {
        Value: map T::AccountId => u32;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn set_value(origin, input: u32) -> Result {
            let sender = ensure_signed(origin)?;
            <Value<T>>::insert(sender, input);
            Ok(())
        }
    }
}
```

If we look at the `decl_storage!`, we can see that we have now defined a mapping from an `AccountId` to a `u32`. This means that each `AccountId` will be able to store and keep their own unique value independent from everyone else.

We also updated our `set_value` function to support this new storage, taking advantage of the `sender` value that we ignored before and calling `insert()` with our `(key, value)` pair.

For this new functionality, we needed to change the `srml_support::StorageValue` to `StorageMap`.

## Checking Our Work in the Polkadot UI

Even though this code should compile without errors, now would be a good time to check out work.

After running:

```
./build.sh
cargo build --release
substrate purge-chain --dev
```

we can start our node:

```
./target/release/substrate-cryptokitties --dev
```

If we go back into the [Polkadot UI](https://polkadot.js.org/apps), we should see evidence of our node producing blocks.

Go to the **Extrinsics** tab

[TODO: Write/copy instructions to test]

---
**Learn More**

Talk about funding accounts to enable transactions

[TODO: make this a page]

---