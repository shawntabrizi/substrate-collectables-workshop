pub trait Trait: balances::Trait {}

decl_storage! {
    trait Store for Module<T: Trait> as CryptokittiesStorage {
          // Declare storage and getter functions here
  }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Declare public functions here
    }
}