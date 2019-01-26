use srml_support::StorageValue;
// ACTION: import any required libraries here
//      HINT: Use errors from the Rust compiler to help you

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