Storing a Structure
===

If you thought everyone getting their own number was cool, lets try to give them all digital kitties!

First we need to define what properties our kitties have in the form of a `struct`, and then we need to learn how to store these custom `structs` in our runtime storage.

## Defining a Custom Struct
You can define a custom struct for your runtime like so:

```
#[derive(Encode, Decode, Default, Clone, PartialEq, Debug)]
pub struct IOU<Balance, AccountId> {
    id: u32,
    amount: Balance,
    who: AccountId,
}
```

[TODO: Fix This]

The `name` and `price` properties of a kitty should be self-explanatory. The `id` hash is a unique, permanent identifier we will use for a kitty. `dna` will be used to store the specific physical characteristics a cat has. Finally, we will introduce a `gen` properity, which will keep track of what generation each kitty is once we introduce breeding.

### Using Generics
Note that we define our struct using [*generics*](https://doc.rust-lang.org/rust-by-example/generics.html), which is a powerful concept in Rust that we will not dive deep into. For the purposes of this tutorial, you can assume these are simply aliases to Substrate specific types like `system::Trait::Hash` and `balances::Trait::Balance`.

There is one last thing we are going to glaze over, which is the `derive` macro used at the very top. In order for custom structs to be used throughout the Substrate codebase, they must satisfy a set of `traits` about what can and cannot be done to that type. This macro, automatically generates the required code to give this struct the ability to be encoded, decoded, cloned, etc... for the purposes of this tutorial you can treat it like magic. (because it is...)

## Using a Custom Struct in a Runtime

Disclaimers out of the way, we can finally plug this kitty into our runtime:

### Custom Struct in Storage
```
decl_storage! {
    trait Store for Module<T: Trait> as Example {
        OwnedKitty: map T::AccountId => Kitty<T::Hash, T::Balance>;
    }
}
```

### Custom Struct in Function
```
decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn create_kitty(origin, name: Vec<u8>) -> Result {
            let sender = ensure_signed(origin)?;

            let new_kitty = Kitty {
                                id: <T as system::Trait>::Hashing::hash_of(&0),
                                name: name,
                                dna: <T as system::Trait>::Hashing::hash_of(&0),
                                price: <T::Balance as As<u64>>::sa(0),
                                gen: 0,
                            };

            <OwnedKitty<T>>::insert(sender, new_kitty);
            Ok(())
        }
    }
}
```

You can see that when the user called our new `create_kitty()` function, we generate a `new_kitty` using the `Kitty` struct, and then insert that directly into our storage like before. The kitty's data is just filler information for now, but we will fix that later.

Note that our storage has been renamed to `OwnedKitty` and is now configured to store a `Kitty` for each `AccountId`.

Substrate does not directly support `Strings`, so we are using a `Vec<u8>` to act as one for our kitties. We will need to remember to convert this value to and from a human readble string with our front end UI.

## Initializing Substrate Types

[TODO: May not be needed]

## Your Turn

Update your storage mapping runtime to store a `Kitty` struct instead of a u64.

A `Kitty` should have the following properties:

 - `id` : `T::Hash`
 - `name` : `Vec<u8>`
 - `dna` : `T::Hash`
 - `price` : `T::Balance`
 - `gen` : `u64`

Then modify your `set_value`

---
**Learn More**

add learn more

[TODO: make this a page]

---