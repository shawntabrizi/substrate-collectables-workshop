Owning Multiple Kitties
===

Our naive impementation of kitty creation allowed each user to have a single kitty to their name. But if we want to really build and extend our game, we need to support users each having and managing multiple kitties.

Do enable this, we will introduce a few new storage items used for tracking kitties and their owner, and we will modify our existing mapping to support this functionality.

Here is a look at our final storage structure:

```
decl_storage! {
    trait Store for Module<T: Trait> as CryptokittiesStorage {
        // Look up a kitty by its owner and relative index
        OwnedTokens get(token_of_owner_by_index): map (T::AccountId, u64) => Kitty<T::Hash, T::Balance>;

        // Total number of kitties
        TotalSupply get(total_supply): u64;
        
        // Get the owner of a kitty by its global index
        TokenOwner get(owner_of): map u64 => Option<T::AccountId>;

        // Look up a kitty by its global index
        AllTokens get(token_by_index): map u64 => Kitty<T::Hash, T::Balance>;

        // Get the number of kitties owned by a user
        OwnedTokensCount get(balance_of): map T::AccountId => u64;

        // Convert a kitties global index to its owners relative index
        OwnedTokenIndex: map u64 => u64;
    }
}
```

Rather than have a mapping which points from the user directly to a specific `Kitty` object, we now have the tuple of an Owner and a relative index which points to a kitty in `OwnedTokens`. This is really the same as a two dimensional array: OwnedTokens[owner][index].

We also added some other storage items which will be used as exposed getter functions for our runtime. `AllTokens` will keep track of each kitty and its global index, and allow us to easily iterate throught the full list of kitties created. `OwnedTokensCount` will keep track of how many kitties each users owns so that we know the valid inputs for `OwnedTokens`.

Finally we introduce an `OwnedTokenIndex` which tracks a kitties *relative* index from its global index. Basically, we do not want to run a loop having to look for a specific kitty in the `OwnedTokens` for things like transferring kitties or burning kitties (OH NO!). So we use a little storage space to save a ton of computation :)

We will need to update our `create_kitty()` logic again to manage these storage items. Let's remember to follow the "**check, then store**" pattern we talked about before.

[TODO: Add borrows to all logic?]

Our logic should look like this:

```
decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn create_kitty(origin, name: Vec<u8>) -> Result {
            let sender = ensure_signed(origin)?;

            let total_supply = Self::total_supply();
            let balance_of = Self::balance_of(&sender);

            let new_total_supply = match total_supply.checked_add(1) {
                Some(x) => x,
                None => return Err("Overflow when adding to total supply"),
            };

            let new_balance_of = match balance_of.checked_add(1) {
                Some(x) => x,
                None => return Err("Overflow when adding to user balance"),
            };

            let new_kitty = Kitty {
                                name: name,
                                dna: <T as system::Trait>::Hashing::hash_of(&0),
                                price: <T::Balance as As<u64>>::sa(0),
                                gen: 0,
                            };

            <OwnedTokens<T>>::insert((sender.clone(), balance_of), &new_kitty);
            <TotalSupply<T>>::put(new_total_supply);
            <TokenOwner<T>>::insert(total_supply, &sender);
            <AllTokens<T>>::insert(total_supply, &new_kitty);
            <OwnedTokensCount<T>>::insert(&sender, new_balance_of);
            <OwnedTokenIndex<T>>::insert(total_supply, balance_of);

            Ok(())
        }
    }
}
```

Nothing here should be too surprising... its just like doing some taxes.

---
**Learn More**

Talk about ERC721 and our naming choices...

[TODO: make this a page]

---