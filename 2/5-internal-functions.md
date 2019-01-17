Internal Functions
===

Our `create_kitty()` function is pretty bulky and has code we will probably want to use later when we introduce kitty breeding and other ways to "mint" new kitties.

We will take this opportunity to teach you about writing internal functions which are not directly exposed through your runtime's API, but are still accessible by your module.

These internal functions can be placed inside of an implementation block for your `Module` which is placed inside your rust file:

```
impl<T: Trait> Module<T> {
    // Your internal functions here
}
```

For our refactor, we want to seperate the logic which generates our `Kitty` object from the logic that mints the unique token id (`T::Hash`) used that we track and manage.

There is no new code here, so we will show you our full module, refactored in this way:

```
use parity_codec::Encode;
use srml_support::{StorageValue, StorageMap, dispatch::Result};
use system::ensure_signed;
use runtime_primitives::traits::{As, Hash};
use rstd::prelude::*;

#[derive(Encode, Decode, Default, Clone, PartialEq, Debug)]
pub struct Kitty<Hash, Balance> {
    id: Hash,
    name: Vec<u8>,
    dna: Hash,
    price: Balance,
    gen: u64,
}

decl_event!(
    pub enum Event<T>
    where
        <T as system::Trait>::AccountId,
        <T as system::Trait>::Hash
    {
        Created(AccountId, Hash),
        Transferred(AccountId, AccountId, Hash),
        Bought(AccountId, AccountId, Hash),
        PriceSet(AccountId, Hash),
    }
);

decl_storage! {
    trait Store for Module<T: Trait> as CryptokittiesStorage {
        Kitties get(kitty): map T::Hash => Kitty<T::Hash, T::Balance>;
        KittyOwner get(owner_of): map T::Hash => Option<T::AccountId>;

        AllKitties get(kitty_by_index): map u64 => T::Hash;
        AllKittiesCount get(all_kitties_count): u64;
        AllKittiesIndex: map T::Hash => u64;

        OwnedKitties get(kitty_of_owner_by_index): map (T::AccountId, u64) => T::Hash;
        OwnedKittiesCount get(owned_kitty_count): map T::AccountId => u64;
        OwnedKittiesIndex: map T::Hash => u64;
        
        Nonce: u64;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn create_kitty(origin, name: Vec<u8>) -> Result {
            let sender = ensure_signed(origin)?;
            let nonce = <Nonce<T>>::get();
            let random_hash = (<system::Module<T>>::random_seed(), &sender, nonce)
                                .using_encoded(<T as system::Trait>::Hashing::hash);

            let new_kitty = Kitty {
                                id: random_hash,
                                name: name,
                                dna: random_hash,
                                price: <T::Balance as As<u64>>::sa(0),
                                gen: 0,
                            };

            Self::_mint(sender, random_hash)?;
            <Kitties<T>>::insert(random_hash, new_kitty);
            <Nonce<T>>::mutate(|n| *n += 1);

            Ok(())
        }
    }
}

impl<T: Trait> Module<T> {
    fn _mint(to: T::AccountId, kitty_id: T::Hash) -> Result {
        ensure!(!<KittyOwner<T>>::exists(kitty_id), "Kitty already exists");

        let owned_kitty_count = Self::owned_kitty_count(&to);

        let new_owned_kitty_count = match owned_kitty_count.checked_add(1) {
            Some(c) => c,
            None => return Err("Overflow adding a new kitty to account balance"),
        };

        let all_kitties_count = Self::all_kitties_count();

        let new_all_kitties_count = match all_kitties_count.checked_add(1) {
            Some (c) => c,
            None => return Err("Overflow when adding new kitty to total supply"),
        };

        <KittyOwner<T>>::insert(kitty_id, &to);
        <AllKittiesIndex<T>>::insert(kitty_id, all_kitties_count);
        <AllKitties<T>>::insert(all_kitties_count, kitty_id);
        <AllKittiesCount<T>>::put(new_all_kitties_count);
        <OwnedKittiesIndex<T>>::insert(kitty_id, owned_kitty_count);
        <OwnedKitties<T>>::insert((to, owned_kitty_count), kitty_id);
        <OwnedKittiesCount<T>>::insert(&to, new_owned_kitty_count);

        Self::deposit_event(RawEvent::Created(to, kitty_id));

        Ok(())
    }
}
```