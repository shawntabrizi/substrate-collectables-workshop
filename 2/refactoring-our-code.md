Refactoring Our Code
===

Our `create_kitty()` function is pretty bulky and has code we will probably want to use later when we introduce kitty breeding and other ways to "mint" new kitties.

We will take this opportunity to teach you about writing private internal functions which are not directly exposed through your runtime's API, but are still accessible by your module.

## Public Interfaces and Private Functions

Within your runtime, you are able to include an implementation of your runtime module like so:

```rust
impl<T: Trait> Module<T> {
    // Your functions here
}
```

Functions in this block are usually public interfaces or private functions. Public interfaces should be labeled `pub` and generally fall into inspector functions that do not write to storage and operation functions that do. Private functions are your usual private utilities unavailable to other modules.

You can call functions defined here using the `Self::function_name()` pattern you have seen before. Here is an intentionally overcomplicated example:

```rust
decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn adder_to_storage(origin, num1: u32, num2: u32) -> Result<(), &'static str> {
            let _sender = ensure_signed(origin)?;
            let result = Self::_adder(num1, num2)?;

            Self::_store_value(result)?;

            Ok(())
        }
    }
}

impl<T: Trait> Module<T> {
    fn _adder(num1: u32, num2: u32) -> Result<u32, &'static str> {
        num1.checked_add(num2).ok_or("Overflow when adding")
    }

    fn _store_value(value: u32) -> Result<(), &'static str> {
        <MyStorage<T>>::put(value);

        Ok(())
    }
}
```

Remember that we still need to follow a "verify first, write last"  pattern, so it is important to not daisy chain private functions which do writes to storage where there is a chance one will throw an error.

## Your Turn!

For us, the process of creating a new kitty from a `Kitty` object and updating all the storage variables is something that we should make reusable so that we can create other kitties whilst taking advantage of the same code.

Whereas the `create_kitty()` function assumes that the kitty generated will always be given to the user who submitted the transaction, our reusable `mint()` function should not make that assumption.

The `create_kitty()` function also generates the property of the new kitty, but we should not move this to the `mint()` function. There may be multiple ways to generate a kitty object in the future. This function should only ensure that the kitty being created is unique, and that all the storage items are properly updated.

Our template provides the function declaration for `mint()`, and will guide you to refactor the code and update any variables to match the new schema. It might be good to double check your work in the Polkadot-JS Apps UI once again before you move on.

<!-- tabs:start -->

#### ** Template **

[embedded-code](./assets/2.6-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](./assets/2.6-finished-code.rs ':include :type=code embed-final')

#### ** Previous Chapter Solution **

[embedded-code-previous](./assets/2.5-finished-code.rs ':include :type=code embed-previous')

<!-- tabs:end -->
