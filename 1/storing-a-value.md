Storing a Value
===

Now that we have our storage value declared in our runtime, we can actually create a function to push a value to it.

## Declaring a Public Function

We need to define runtime functions that will set and modify our storage values. This can be done within our `decl_module!` macro, which declares all the entry points that your module handles.

Here is an example of an exposed function declaration:

```rust
// Add these imports: 
//
// use support::{dispatch::Result, StorageValue};
// use system::ensure_signed;
decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        fn my_function(origin, input_bool: bool) -> Result {
            let _sender = ensure_signed(origin)?;

            <MyBool<T>>::put(input_bool);
            
            Ok(())
        }
    }
}
```

## Function Structure

Module functions exposed here should always take the form:

```rust
fn foo(origin, bar: Bar, baz: Baz, ...) -> Result;
```

### Origin

The first argument of these functions is always `origin`. `origin` contains information about where the call originated from. This is generally split into three groups:

- Public calls that are signed by an external account.
- Root calls that are allowed to be made only by the governance system.
- Inherent calls that are allowed to be made only by the block authors and validators.

Refer to definition of [Origin](https://substrate.dev/docs/en/overview/glossary#origin) in the Substrate Glossary.

### Result

Additionally, these functions must return the [`Result` type](https://substrate.dev/rustdocs/v1.0/srml_support/dispatch/result/index.html) from the `support::dispatch` module. This means that a successful function call will always return `Ok(())`, otherwise, the logic should catch any errors which may cause a problem and return an `Err()`.

Since these are dispatched functions, there are two extremely important things to remember:

- MUST NOT PANIC: Under no circumstances (save, perhaps, storage getting into an irreparably damaged state) must this function panic.
- NO SIDE-EFFECTS ON ERROR: This function must either complete totally and return `Ok(())`, or it must have no side-effects on storage and return `Err('Some reason')`.

We will talk about these more later. Throughout this tutorial, we will make sure that both of these conditions are satisfied, and we will remind you to do the same.

## Checking for a Signed Message

As mentioned, the first argument in any of these module functions is the `origin`. There are three convenience calls in `system` that do the matching for you and return a convenient result: `ensure_signed`, `ensure_root` and `ensure_inherent`. **You should always match against them as the first thing you do in your function.**

We can use the `ensure_signed()` function from `system::ensure_signed` to check the origin, and "ensure" that the messaged is signed by a valid account. We can even derive the signing account from the result of the function call as shown in the example function above.

## Your Turn!

Use the template to create a `set_value()` function which will allow a user to send a signed message which puts a `u64` into the runtime storage.

<!-- tabs:start -->

#### ** Template **

[embedded-code](./assets/1.3-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](./assets/1.3-finished-code.rs ':include :type=code embed-final')

#### ** Previous Chapter Solution **

[embedded-code](./assets/1.2-finished-code.rs ':include :type=code embed-previous')

<!-- tabs:end -->

---
**Learn More**

If you try to compile the code samples we showed above without importing the required libraries, you will get some errors:

```rust
error[E0425]: cannot find function `ensure_signed` in this scope
  --> src/substratekitties.rs:15:27
   |
15 |             let _sender = ensure_signed(origin)?;
   |                           ^^^^^^^^^^^^^ not found in this scope
help: possible candidate is found in another module, you can import it into scope
   |
2  | use system::ensure_signed;
   |
```

As you can see, we added some functionality in our code that we had not yet imported into our module. Rust actually can help us here by suggesting ways to solve these problems. If we listen to Rust, then we can simply add these `use` statements at the very top to get our code to compile again:

```rust
use system::ensure_signed;
```

As mentioned in "common patterns" section, Rust will be your friend throughout runtime development, and *should mostly* help you overcome any issues in your code. Moving forward we will try to mention whenever you need to import a new library, but don't be worried when the compiler sends you some errors. Instead embrace the helpful suggestions it might be giving you.

---
