Storage Mapping
===

Our last runtime only allowed us to store a single value across all users of our blockchain. As we start thinking toward our collectables chain, it makes sense to add support for each user to have their own value stored.

To enable this, we will replace our simple single value storage with a storage mapping.

## Substrate Specific Types

Before we jump into storage mappings, let's talk about some substrate specific types we will be using.

Your default runtime template includes a bunch of modules that expose types that you would expect to get from a blockchain. You might even find yourself exposing new types to other parts of the runtime as you do more module development.

In this tutorial we will only be using 3 substrate specific types:

 1. AccountId
 2. Balance
 3. Hash

Our module does not natively have access to these types, but we can easily gain access by having our module's `Trait` inherit from the modules that defined these types. In this case the `balances` module has everything we need:

```rust
pub trait Trait: balances::Trait {}
```

We can access these types anywhere we have specified the generic `<T: Trait>` using `T::Type` like we have done in the example above.

## Declaring a Storage Map

A Storage Map allows you to put a basic (key, value) pair into your runtime storage. It can be declared like this:

```rust
decl_storage! {
    trait Store for Module<T: Trait> as Example {
        SomeValue get(some_value_getter): map u32 => u32;
        MyValue: map T::AccountId => u32;
    }
}
```

You can see that mappings can be pretty useful when you want to represent "owned" data. Since we can create a mapping from a user (`AccountId`) to some value such as with `MyValue`, we can keep in storage information about that user. We can even build logic in our runtime which manages who is allowed to modify those values, a common pattern we will be using throughout this tutorial.

To use a storage map, you will need to import the `support::StorageMap` type.

### Working with a StorageMap

The functions used to access a `StorageMap` are in [the same place as `StorageValue`](https://substrate.dev/rustdocs/v1.0/srml_support/storage/trait.StorageMap.html):

```rust
/// Get the prefix key in storage.
fn prefix() -> &'static [u8];

/// Get the storage key used to fetch a value corresponding to a specific key.
fn key_for(x: &K) -> Vec<u8>;

/// true if the value is defined in storage.
fn exists<S: Storage>(key: &K, storage: &S) -> bool {
    storage.exists(&Self::key_for(key)[..])
}

/// Load the value associated with the given key from the map.
fn get<S: Storage>(key: &K, storage: &S) -> Self::Query;

/// Take the value under a key.
fn take<S: Storage>(key: &K, storage: &S) -> Self::Query;

/// Store a value to be associated with the given key from the map.
fn insert<S: Storage>(key: &K, val: &V, storage: &S) {
    storage.put(&Self::key_for(key)[..], val);
}

/// Remove the value under a key.
fn remove<S: Storage>(key: &K, storage: &S) {
    storage.kill(&Self::key_for(key)[..]);
}

/// Mutate the value under a key.
fn mutate<R, F: FnOnce(&mut Self::Query) -> R, S: Storage>(key: &K, f: F, storage: &S) -> R;
```

So if you want to "insert" a value into a StorageMapping, you need to provide the key and value like so:

```rust
<SomeValue<T>>::insert(key, value);
```

You could then query that value with either:

```rust
let my_value = <SomeValue<T>>::get(key);
let also_my_value = Self::some_value_getter(key);
```

## Your Turn!

Update your simple storage example to now store a map from an `AccountId` to a `u64`.

<!-- tabs:start -->

#### ** Template **

[embedded-code](./assets/1.4-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](./assets/1.4-finished-code.rs ':include :type=code embed-final')

#### ** Previous Chapter Solution **

[embedded-code-previous](./assets/1.3-finished-code.rs ':include :type=code embed-previous')

<!-- tabs:end -->

---
**Learn More**

Talk about funding accounts to enable transactions

[TODO: make this a page]

---
