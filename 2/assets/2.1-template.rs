use srml_support::{StorageValue, StorageMap, dispatch::Result};
use system::ensure_signed;
use runtime_primitives::traits::{As, Hash};
use rstd::prelude::*;

#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct Kitty<Hash, Balance> {
    id: Hash,
    name: Vec<u8>,
    dna: Hash,
    price: Balance,
    gen: u64,
}

pub trait Trait: balances::Trait {}

decl_storage! {
    trait Store for Module<T: Trait> as KittyStorage {
        // ACTION: Add three new storage items: 
        //      - `Kitties` which maps a `T::Hash` to a `Kitty<T::Hash, T::Balance>`
        //      - `KittyOwner` which maps a `T::Hash` to an `Option<T::AccountId>`

        // ACTION: Update `OwnedKitty` to store a `T::Hash`
        OwnedKitty get(kitty_of_owner): map T::AccountId => Kitty<T::Hash, T::Balance>;

        Nonce: u64;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        fn create_kitty(origin, name: Vec<u8>) -> Result {
            let sender = ensure_signed(origin)?;

            // ACTION: Generate a `random_hash` using: 
            //      - `<system::Module<T>>::random_seed()`
            //      - `sender`
            //      - `Nonce`

            // ACTION: "Ensure" our `random_hash` does not collide with an existing token

            // ACTION: Update our Kitty to use this `random_hash` as the `id` and the `dna`
            let new_kitty = Kitty {
                                id: <T as system::Trait>::Hashing::hash_of(&0),
                                name: name,
                                dna: <T as system::Trait>::Hashing::hash_of(&0),
                                price: <T::Balance as As<u64>>::sa(0),
                                gen: 0,
                            };

            // ACTION: "Insert" the storage for `Kitties`, should point from our kitty's id to the `Kitty` object
            // ACTION: "Insert" the storage for `KittyOwner`, should point from our kitty's id to the owner
            // ACTION: Update the OwnedKitty storage below to store the kitty's id rather than the `Kitty` object
            <OwnedKitty<T>>::insert(&sender, new_kitty);

            <Nonce<T>>::mutate(|n| *n += 1);

            Ok(())
        }
    }
}