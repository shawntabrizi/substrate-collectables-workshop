use support::{decl_storage, decl_module, StorageValue, dispatch::Result};
use system::ensure_signed;

pub trait Trait: system::Trait {}

decl_storage! {
    trait Store for Module<T: Trait> as KittyStorage {
        Value: u64;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        fn set_value(origin, value: u64) -> Result {
            let _sender = ensure_signed(origin)?;

            <Value<T>>::put(value);

            Ok(())
        }
    }
}