Owning Multiple Kitties
===

Our naive impementation of kitty creation allowed each user to have a single kitty to their name. But if we want to really build and extend our game, we need to support users each having and managing multiple kitties.

Do enable this, we will introduce our final set of storage items used for tracking multiple kitties per owner.

## Using Tuples to Emulate 

```
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
```

[TODO: All of this explaination text needs updating ]

The first major change that we have made is to reference the unique id for each kitty throughout our storage rather than repeatedly storing our `Kitty` object. Only a single storage item, `Kitties`, actually contains our `Kitty` object. We will design our `id` property so that it is guarenteed to be unique. Thus, as kitties move around, we will have a consistant and effective way to look them up.

Kitty ownership is defined by the `KittyOwner` mapping, and will be used for any authorization checks.

Substrate does not directly support storing indexed list because it can be easily simulated by using a mapping and a counter. We have done this to keep track of `AllKitties` and `OwnedKitties`. These two lists will allow us to display all of the kitties in our system, or all of the kitties that a particular user owns.

Finally we create convenience storage items with `AllKittiesIndex` and `OwnedKittiesIndex` which tracks where a specific kitty is in both of these lists. This allows us to avoid running loops to look for a specific kitty when we need to update data during transfers.

We will need to update our `create_kitty()` logic again to manage these storage items. Let's remember to follow the "**check, then store**" pattern we talked about before.

[TODO: Add borrows to all logic?]

Our logic should look like this:

```
decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn create_kitty(origin, name: Vec<u8>) -> Result {
            let sender = ensure_signed(origin)?;
            let nonce = <Nonce<T>>::get();

            let random_hash = (<system::Module<T>>::random_seed(), &sender, nonce).using_encoded(<T as system::Trait>::Hashing::hash);

            ensure!(!<Kitties<T>>::exists(random_hash), "This kitty id already exists");

            let all_kitties_count = Self::all_kitties_count();

            let new_all_kities_count = match all_kitties_count.checked_add(1) {
                Some(x) => x,
                None => return Err("Overflow when adding to total kitty count"),
            };

            let owned_kitty_count = Self::owned_kitty_count(&sender);

            let new_owned_kitty_count = match owned_kitty_count.checked_add(1) {
                Some(x) => x,
                None => return Err("Overflow when adding to owned kitty count"),
            };

            let new_kitty = Kitty {
                                id: random_hash,
                                name: name,
                                dna: random_hash,
                                price: <T::Balance as As<u64>>::sa(0),
                                gen: 0,
                            };

            <OwnedKitties<T>>::insert((sender.clone(), owned_kitty_count), random_hash);
            <OwnedKittiesIndex<T>>::insert(random_hash, owned_kitty_count);
            <OwnedKittiesCount<T>>::insert(&sender, new_owned_kitty_count);
            <Kitties<T>>::insert(random_hash, new_kitty);
            <KittyOwner<T>>::insert(random_hash, &sender);
            <AllKitties<T>>::insert(all_kitties_count, random_hash);
            <AllKittiesIndex<T>>::insert(random_hash, all_kitties_count);
            <AllKittiesCount<T>>::put(new_all_kities_count);
            <Nonce<T>>::mutate(|n| *n += 1);

            Ok(())
        }
    }
}
```

Nothing here should be too surprising... its just like doing some taxes.

---
**Learn More**

Talk about Option<T::AccountID>

[TODO: make this a page]

---