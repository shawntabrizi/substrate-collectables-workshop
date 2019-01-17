Simple Storage
===

Let's add just about the most simple function we can to our runtime: storing a variable.

We will need to define a function which can store/update the value, and a place for the value to live.

Here is how our new runtime will look:

```
pub trait Trait: balances::Trait {}

decl_storage! {
    trait Store for Module<T: Trait> as CryptokittiesStorage {
        Value: u32;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn set_value(origin, input: u32) -> Result {
            let _sender = ensure_signed(origin)?;
            <Value<T>>::put(input);
            Ok(())
        }
    }
}
```

To start, we have defined a simple `u32` variable called `Value`. Then we created a function called `set_value` which takes in an `input` and `put`s that value into our variable.

You will notice that `set_value` also has an `origin` value as an input. All public functions will have `origin` as the first argument in the function. `origin` can tell the function where the call to that function originated from. In our runtime, the call will always originate from a signed message from a user (a transaction), however, Substrate supports function calls more general than this in the form of unsigned messages (an inherent). Transactions and inherents make up all possibe calls to the runtime, also called extrinsics.

## Importing Required Features

[TODO: Make the sentence above not suck]

When we call `ensure_signed(origin)`, we are just making sure that any call to this function is coming from an account via a signed message. You can see that we can even tell which user sent us the message, but we won't be using that until later.

If you try to compile this code, you will see some errors:

```
error[E0425]: cannot find function `ensure_signed` in this scope
  --> src/cryptokitties.rs:15:27
   |
15 |             let _sender = ensure_signed(origin)?;
   |                           ^^^^^^^^^^^^^ not found in this scope
help: possible candidate is found in another module, you can import it into scope
   |
2  | use system::ensure_signed;
   |

error[E0599]: no function or associated item named `put` found for type `cryptokitties::Value<T>` in the current scope
  --> src/cryptokitties.rs:16:25
   |
6  | decl_storage! {
   | ------------- function or associated item `put` not found for this
...
16 |             <Value<T>>::put(input);
   |             ------------^^^
   |             |
   |             function or associated item not found in `cryptokitties::Value<T>`
   |
   = help: items from traits can only be used if the trait is in scope
   = note: the following traits are implemented but not in scope, perhaps add a `use` for one of them:
           candidate #1: `use srml_support::Storage;`
           candidate #2: `use srml_support::storage::generator::StorageValue;`
           candidate #3: `use srml_support::StorageValue;`

...
```

As you can see, we added use some functionality in our code that we had not yet imported into our module. Rust actually can help us here by suggesting ways to solve these problems. If we listen to Rust, then we can simply add these `use` statements at the very top to get our code to compile again:

```
use system::ensure_signed;
use srml_support::{StorageValue, dispatch::Result};
```

As mentioned in "common patterns" section, Rust will be your friend throughout runtime development, and *should mostly* help you overcome any issues in your code. Moving forward we won't always be so explicit about the new imports needed for new code, but keep an eye out because we will continue to add new features as our functionality increases.

---
**Learn More**

Give a tip about how to use SRML as a template for new ideas

[TODO: make this a page]

---