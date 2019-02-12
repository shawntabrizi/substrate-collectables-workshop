use support::{decl_storage, decl_module, StorageValue};

pub trait Trait: system::Trait {}

decl_storage! {
    trait Store for Module<T: Trait> as KittyStorage {
        Value: u64;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

    }
}