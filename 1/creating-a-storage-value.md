Creating a Storage Value
===

Let's add the most simple logic we can to our runtime: a function which stores a variable.

To do this, we will first need to define a storage variable for a [**Storage Item**](https://substrate.dev/docs/en/overview/glossary#storage-items) in the [**`decl_storage!`**](https://substrate.dev/rustdocs/v1.0/srml_support_procedural/macro.decl_storage.html) macro. This allows for type-safe usage of the Substrate storage database, so you can keep things around between blocks.

## Declaring a Storage Value

Substrate natively supports all the primitive types available in Rust (`bool`, `u8`, `u32`, etc..) and as well some custom types specific to Substrate (`AccountId`, `BlockNumber`, `Hash`, [and more](https://polkadot.js.org/api/types/)...)

You can declare a simple storage item like this:

```rust
decl_storage! {
    trait Store for Module<T: Trait> as Example {
        MyU32: u32;
        MyBool get(my_bool_getter): bool;
    }
}
```

Here we have defined two variables: a `u32` and a `bool` with a getter function named `my_bool_getter`. The `get` parameter is optional, but if you add it to your storage item it will expose a getter function with the name specified (`fn getter_name() -> Type`).

To store these basic storage values, you need to import the `support::StorageValue` module.

### Working with a Storage Value

The functions used to access a `StorageValue` are defined in [`srml_support::storage`](https://substrate.dev/rustdocs/v1.0/srml_support/storage/trait.StorageValue.html):

```rust
/// Get the storage key.
fn key() -> &'static [u8];

/// true if the value is defined in storage.
fn exists<S: Storage>(storage: &S) -> bool {
    storage.exists(Self::key())
}

/// Load the value from the provided storage instance.
fn get<S: Storage>(storage: &S) -> Self::Query;

/// Take a value from storage, removing it afterwards.
fn take<S: Storage>(storage: &S) -> Self::Query;

/// Store a value under this key into the provided storage instance.
fn put<S: Storage>(val: &T, storage: &S) {
    storage.put(Self::key(), val)
}

/// Mutate this value
fn mutate<R, F: FnOnce(&mut Self::Query) -> R, S: Storage>(f: F, storage: &S) -> R;

/// Clear the storage value.
fn kill<S: Storage>(storage: &S) {
    storage.kill(Self::key())
}
```

So if you want to "put" the value of `MyU32`, you could write:

```rust
<MyU32<T>>::put(1337);
```

If you wanted to "get" the value of `MyBool`, you could write either:

```rust
let my_bool = <MyBool<T>>::get();
let also_my_bool = Self::my_bool_getter();
```

We will show you exactly how to integrate these calls into your module in the next section.

## Your Turn!

Create a storage value called `Value` which stores a `u64`.

Make sure to import any required libraries required by the compiler. Your code should compile successfully.

<!-- tabs:start -->

#### ** Template **

[embedded-code](./assets/1.2-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](./assets/1.2-finished-code.rs ':include :type=code embed-final')

#### ** Previous Chapter Solution **

[embedded-code-previous](./assets/1.1-finished-code.rs ':include :type=code embed-previous')

<!-- tabs:end -->
