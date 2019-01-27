use parity_codec::Encode;
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

pub trait Trait: balances::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_event!(
    pub enum Event<T>
    where
        <T as system::Trait>::AccountId,
        <T as system::Trait>::Hash
    {
        Created(AccountId, Hash),
        // ACTION: Create a `PriceSet` event here
        //      HINT: Do you need a new type for this event? (yes)
    }
);

decl_storage! {
    trait Store for Module<T: Trait> as KittyStorage {
        Kitties get(kitty): map T::Hash => Kitty<T::Hash, T::Balance>;
        KittyOwner get(owner_of): map T::Hash => Option<T::AccountId>;

        AllKittiesArray get(kitty_by_index): map u64 => T::Hash;
        AllKittiesCount get(all_kitties_count): u64;
        AllKittiesIndex: map T::Hash => u64;

        OwnedKittiesArray get(kitty_of_owner_by_index): map (T::AccountId, u64) => T::Hash;
        OwnedKittiesCount get(owned_kitty_count): map T::AccountId => u64;
        OwnedKittiesIndex: map T::Hash => u64;
        
        Nonce: u64;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        fn deposit_event<T>() = default;

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

            Self::_mint(sender, random_hash, new_kitty)?;
            
            <Nonce<T>>::mutate(|n| *n += 1);

            Ok(())
        }

        fn set_price(origin, kitty_id: T::Hash, new_price: T::Balance) -> Result {
            let sender = ensure_signed(origin)?;

            // ACTION: Check that the kitty with `kitty_id` exists

            // ACTION: Check if owner exists for `kitty_id`
            //      - If it does, check that `sender` is the `owner`
            //      - If it doesn't, return an `Err()` that no `owner` exists

            let mut kitty = Self::kitty(kitty_id);
            
            // ACTION: Set the new price for the kitty

            // ACTION: Update the kitty in storage

            // ACTION: Deposit a `PriceSet` event with relevant data
            //      - owner
            //      - kitty id
            //      - the new price

            Ok(())
        }
    }
}

impl<T: Trait> Module<T> {
    fn _mint(to: T::AccountId, kitty_id: T::Hash, new_kitty: Kitty<T::Hash, T::Balance>) -> Result {
        ensure!(!<KittyOwner<T>>::exists(kitty_id), "Kitty already exists");

        let owned_kitty_count = Self::owned_kitty_count(&to);

        let new_owned_kitty_count = match owned_kitty_count.checked_add(1) {
            Some(c) => c,
            None => return Err("Overflow adding a new kitty to account balance"),
        };

        let all_kitties_count = Self::all_kitties_count();

        let new_all_kitties_count = match all_kitties_count.checked_add(1) {
            Some (c) => c,
            None => return Err("Overflow adding a new kitty to total supply"),
        };

        <Kitties<T>>::insert(kitty_id, new_kitty);
        <KittyOwner<T>>::insert(kitty_id, &to);

        <AllKittiesArray<T>>::insert(all_kitties_count, kitty_id);
        <AllKittiesCount<T>>::put(new_all_kitties_count);
        <AllKittiesIndex<T>>::insert(kitty_id, all_kitties_count);

        <OwnedKittiesArray<T>>::insert((to.clone(), owned_kitty_count), kitty_id);
        <OwnedKittiesCount<T>>::insert(&to, new_owned_kitty_count);
        <OwnedKittiesIndex<T>>::insert(kitty_id, owned_kitty_count);

        Self::deposit_event(RawEvent::Created(to, kitty_id));

        Ok(())
    }
}