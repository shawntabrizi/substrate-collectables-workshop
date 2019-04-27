// ACTION: To store a value, you need to import `support::StorageValue`
//   HINT: You can simply add `StorageValue` to the list below since
//         all these dependencies import from `support`.
use support::{decl_storage, decl_module};

pub trait Trait: system::Trait {}

decl_storage! {
    trait Store for Module<T: Trait> as KittyStorage {
        // ACTION: create a storage item named `Value` which stores a `u64`
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

    }
}