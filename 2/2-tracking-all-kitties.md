Tracking All Kitties
===

Now that we have enabled each user to create their own unique kitty, we should probably start tracking them!

It makes sense for the our game to have track a total number of kitties created, as well as a way to track who owns each kitty.

We can add two new storage items for that like so:

```
decl_storage! {
    trait Store for Module<T: Trait> as CryptokittiesStorage {
        OwnedKitty get(kitty_of_owner): map T::AccountId => T::Hash;
        Kitties get(kitty): map T::Hash => Kitty<T::Hash, T::Balance>;
        
        AllKittiesCount get(all_kitties_count): u64;
        KittyOwner get(owner_of): map u64 => T::AccountId;

        Nonce: u64;
    }
}
```

Note that we have introduced *getter* functions in the form of `get(function_name)` for some of these variables. These getter functions can be used in our module, but more importantly, are exposed as a part of our runtime's API.

These getter functions are the only way to directly return a value from the runtime and do not allow you to transform the value in the storage before it is returned.

Let's now update our `create_kitty` function to support updating these new storage items when a kitty gets created.

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

            let new_kitty = Kitty {
                                id: random_hash,
                                name: name,
                                dna: random_hash,
                                price: <T::Balance as As<u64>>::sa(0),
                                gen: 0,
                            };

            <OwnedKitty<T>>::insert(sender, random_hash);
            <Kitties<T>>::insert(random_hash, new_kitty);
            <AllKitties<T>>::insert(all_kitties_count, random_hash);
            <AllKittiesIndex<T>>::insert(random_hash, all_kitties_count);
            <AllKittiesCount<T>>::put(new_all_kities_count);
            <Nonce<T>>::mutate(|n| *n += 1);

            Ok(())
        }
    }
}
```

There are a few things to take note of here. First, we are incrementing the `AllKittiesCount` whenever the `create_kitty()` function is called. Even though `AllKittiesCount` is a `u64`, and it would take a massive number of kitties to cause an issue, we should still be checking for overflows to ensure our runtime's state never goes bad.

To do this, we called [`checked_add()`](https://doc.rust-lang.org/std/primitive.u64.html#method.checked_add), which is a standard rust function that will return `Some` value if the addition works or `None` if an overflow occurs. In this case, we assign `new_all_kitties_count` the new value in the case of success, or we return an error and stop the runtime execution if it fails. We will be able to see this error message as an output in our node's terminal if it occurs.

We also push our new kitty into `AllKitties` and `AllKittiesIndex`.
[TODO: Make this better]

[TODO: Mention this in an earlier section]

Next, you should note that only at the very end of our function do we actually update the values in our storage. Unlike Ethereum, if a transaction returns an error at some point, storage modifictions are not reverted. Thus it is very important for your runtime to follow a **"check, then store"** pattern.

[TODO: there may be a better way to write this info]

This updated code should compile for you, but have you noticed a problem yet? What would happen if the same user called the `create_kitty()` function?

Right now users can only have one kitty, but our total supply will keep going up. Let's fix that by enabling users to own multiple kitties!

---
**Learn More**

Talk about cloning and borrowing...

[TODO: make this a page]

---