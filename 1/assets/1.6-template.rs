use srml_support::{StorageMap, dispatch::Result};
use system::ensure_signed;
use runtime_primitives::traits::As;
// ACTION: Continue to add required libraries here

#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct Kitty<Hash, Balance> {
    // ACTION: Define the properties of your kitty struct here
    //      - `id` as a `Hash`
    //      - `dna` as a `Hash`
    //      - `price` as a `Balance`
    //      - `gen` as a `u64`
}

pub trait Trait: balances::Trait {}

decl_storage! {
    trait Store for Module<T: Trait> as KittyStorage {
        // ACTION: Update this storage item to store a `Kitty<T::Hash, T::Balance>`
        Value: map T::AccountId => u64;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        fn create_kitty(origin) -> Result {
            let sender = ensure_signed(origin)?;

            // ACTION: Create a `Kitty` object named `new_kitty` here
            //      HINT: You can generate a hash with `<T as system::Trait>::Hashing::hash_of(&0)`
            //            And you can generate a `0` balance with `<T::Balance as As<u64>>::sa(0)`

            // ACTION: Store your `new_kitty` into the runtime storage

            Ok(())
        }
    }
}