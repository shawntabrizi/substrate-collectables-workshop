Storage Mapping
===

Unfortunately, our last runtime only allowed us to store a single value across all users of our blockchain. As we build toward our collectables chain, it will make sense for each user to own their own values.

For this we can replace our simple single value storage with a storage mapping.

## Declaring a Storage Map

A Storage Map allows you to put a basic (key, value) pair into your runtime storage. It can be declared like so:

```
decl_storage! {
    trait Store for Module<T: Trait> as Example {
        SomeValue: map u32 => u32;
        MyValue: map T::AccountId => u32;
    }
}
```

To use a storage map, you will need to import the `support::StorageMap` type.

`MyValue` in the example above defines a mapping from an `AccountId` to a `u32`. This means that each `AccountId` key will be able to store and keep their own unique value independent from everyone else. `AccountId` is one of the many types introduced by the default Library of Substrate Runtime Modules (SRML). You will see us use more of them as this tutorial continues.

We also updated our `set_value` function to support this new storage, taking advantage of the `sender` value that we ignored before and calling `insert()` with our `(key, value)` pair.

For this new functionality, we needed to change the `srml_support::StorageValue` to `StorageMap`.

### Working with a StorageMap

The functions used to access a `StorageMap` are in [the same place as `StorageValue`](https://github.com/paritytech/substrate/blob/master/srml/support/src/storage/generator.rs#L162):

```
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

```
<SomeValue<T>>::insert(key, value);
```

## Your Turn!


---
**Learn More**

Talk about funding accounts to enable transactions

[TODO: make this a page]

---