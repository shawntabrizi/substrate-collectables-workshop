use srml_support::dispatch::Result;
// ACTION: import any required libraries here
//      HINT: Use errors from the Rust compiler to help you

pub trait Trait: balances::Trait {}

decl_storage! {
    trait Store for Module<T: Trait> as KittyStorage {
        // ACTION: create a storage item named `Value` which stores a `u64`
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        fn set_value(origin, value: u64) -> Result {
            // ACTION: "Ensure" that the transaction is signed

            // ACTION: "Put" the value into storage
            
            Ok(())
        }
    }
}