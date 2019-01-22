// ACTION: Update StorageValue to StorageMap to support mappings
use srml_support::{StorageValue, dispatch::Result};
use system::ensure_signed;

// ACTION: Update this to use `balances::Trait` to access T::AccountId
pub trait Trait: system::Trait {}

decl_storage! {
    trait Store for Module<T: Trait> as KittyStorage {
        // ACTION: Update this storage item to be a `map` from `T::AccountId` to `u64`
        Value: u64;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        fn set_value(origin, value: u64) -> Result {
            let sender = ensure_signed(origin)?;

            // ACTION: Update this to `insert()` the key/value pair (sender, value)
            <Value<T>>::put(value);

            Ok(())
        }
    }
}