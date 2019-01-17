Generating Random Data
===

At the end of the last section, we allowed each user to create their own kitty. However, they weren't very unique...

If we want to be able to tell these kitties apart, we need to start giving them unique traits! Specifically, we need to generate a unique `id` for each kitty and some random `dna`. This means we need to securely fetch some randomness from our chain. Fortunately, we provide this functionality within our `system` module:

```
<system::Module<T>>::random_seed()
```

Substrate uses a safe mixing algorithm that uses the entropy of previous blocks to generate new random data per block. However, since it is dependent on previous blocks, it can takes ~50-100 blocks to fully warm up and you may notice the seed will not change until then.

To address this, and to address multiple transactions occuring on the same block where the `random_seed()` is the same, we will want to mix this data with other variables like the `sender` and a `nonce`.

A random number generator on substrate will look something like this:

```
let sender = ensure_signed(origin)?;
let nonce = <Nonce<T>>::get();

let random_hash = (<system::Module<T>>::random_seed(), sender, nonce).using_encoded(<T as system::Trait>::Hashing::hash);

<Nonce<T>>::mutate(|n| *n += 1);
```

`Nonce` will be a new item in our storage which we will simply increment whenever we use it.

We can use this `random_hash` to populate both the `id` and `dna` for our kitty, however we want to make sure that `id` is always unique, so we will need to create a storage item which tracks all of the `id`s that have already been used. We will call this `Kitties` and it will track all of the kitties every produced.

With this in mind, lets try to update our module to generate our kitties some unique data:

```
use system::ensure_signed;
use srml_support::{StorageValue, StorageMap, dispatch::Result};
use runtime_primitives::traits::{Hash, As};
use rstd::prelude::*;
use parity_codec::Encode;

pub trait Trait: balances::Trait {}

#[derive(Encode, Decode, Default, Clone, PartialEq, Debug)]
pub struct Kitty<Hash, Balance> {
    id: Hash,
    name: Vec<u8>,
    dna: Hash,
    price: Balance,
    gen: u64,
}

decl_storage! {
    trait Store for Module<T: Trait> as CryptokittiesStorage {
        OwnedKitty: map T::AccountId => T::Hash;
        Kitties: map T::Hash => Kitty<T::Hash, T::Balance>;
        Nonce: u64;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn create_kitty(origin, name: Vec<u8>) -> Result {
            let sender = ensure_signed(origin)?;
            let nonce = <Nonce<T>>::get();

            let random_hash = (<system::Module<T>>::random_seed(), &sender, nonce).using_encoded(<T as system::Trait>::Hashing::hash);

            ensure!(!<Kitties<T>>::exists(random_hash), "This kitty id already exists");

            let new_kitty = Kitty {
                                id: random_hash,
                                name: name,
                                dna: random_hash,
                                price: <T::Balance as As<u64>>::sa(0),
                                gen: 0,
                            };

            <OwnedKitty<T>>::insert(sender, random_hash);
            <Kitties<T>>::insert(random_hash, new_kitty);
            <Nonce<T>>::mutate(|n| *n += 1);
            
            Ok(())
        }
    }
}
```

One thing you might notice about this implementation is that we changed `OwnedKitty` so that it maps to the unique `id` we generated rather than the `Kitty` object. We do this to avoid having two copies of the `Kitty` object in storage, as we can always get the full object by referencing the unique `id`.