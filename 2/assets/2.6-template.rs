use parity_codec::Encode;
use support::{decl_storage, decl_module, StorageValue, StorageMap,
    dispatch::Result, ensure, decl_event};
use system::ensure_signed;
use runtime_primitives::traits::{As, Hash};

#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct Kitty<Hash, Balance> {
    id: Hash,
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

        fn create_kitty(origin) -> Result {
            let sender = ensure_signed(origin)?;

            // ACTION: Move this `owned_kitty_count` and `new_owned_kitty_count` logic into the `_mint()` function
            let owned_kitty_count = Self::owned_kitty_count(&sender);

            let new_owned_kitty_count = owned_kitty_count.checked_add(1)
                .ok_or("Overflow adding a new kitty to account balance")?;

            // ACTION: Move this `all_kitties_count` and `new_all_kitties_count` logic into the `_mint()` function
            let all_kitties_count = Self::all_kitties_count();

            let new_all_kitties_count = all_kitties_count.checked_add(1)
                .ok_or("Overflow adding a new kitty to total supply")?;

            // `nonce` and `random_hash` generation can stay here
            let nonce = <Nonce<T>>::get();
            let random_hash = (<system::Module<T>>::random_seed(), &sender, nonce)
                .using_encoded(<T as system::Trait>::Hashing::hash);

            // ACTION: Move this collision check to the `_mint()` function
            ensure!(!<KittyOwner<T>>::exists(random_hash), "Kitty already exists");

            // Creating the `Kitty` object can stay here
            let new_kitty = Kitty {
                id: random_hash,
                dna: random_hash,
                price: <T::Balance as As<u64>>::sa(0),
                gen: 0,
            };

            // ACTION: Move all of the kitty related storage updates to the `_mint()` function
            <Kitties<T>>::insert(random_hash, new_kitty);
            <KittyOwner<T>>::insert(random_hash, &sender);

            <AllKittiesArray<T>>::insert(all_kitties_count, random_hash);
            <AllKittiesCount<T>>::put(new_all_kitties_count);
            <AllKittiesIndex<T>>::insert(random_hash, all_kitties_count);

            <OwnedKittiesArray<T>>::insert((sender.clone(), owned_kitty_count), random_hash);
            <OwnedKittiesCount<T>>::insert(&sender, new_owned_kitty_count);
            <OwnedKittiesIndex<T>>::insert(random_hash, owned_kitty_count);
            
            // Nonce update can stay here
            <Nonce<T>>::mutate(|n| *n += 1);

            // ACTION: Move this event to the `_mint()` function
            Self::deposit_event(RawEvent::Created(sender, random_hash));

            Ok(())
        }
    }
}

impl<T: Trait> Module<T> {
    fn _mint(to: T::AccountId, kitty_id: T::Hash, new_kitty: Kitty<T::Hash, T::Balance>) -> Result {
        // ACTION: Refactored code goes here
        //      NOTE: Some variables have been renamed to generalize the function
        //
        // In the end, you should remember add the following call to `create_kitty()`:
        // `Self::_mint(sender, random_hash, new_kitty)?;`
        //      HINT: This `_mint()` function has checks which could fail AND will write to storage, so place it carefully

        Ok(())
    }
}