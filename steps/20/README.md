# Storage Maps

Now that you have learned everything you need to know about `StorageValue`s, it is time to move on to `StorageMap`s.

`StorageMap`s are key / value storage items designed for Pallet development.

## Syntax

Declaring a new [`StorageMap`](https://docs.rs/frame-support/38.0.0/frame_support/storage/types/struct.StorageMap.html) is very similar to a `StorageValue`:

```rust
#[pallet::storage]
pub(super) type Kitties<T: Config> = StorageMap<Key = [u8; 32], Value = ()>;
```

Nearly everything is the same, except you need to define a `Key` and `Value`.

Each `Key` can store a separate `Value`, which makes maps super useful.

In this case `[u8; 32]` represents some unique identifier for each Kitty we will store, and `()` is simply a placeholder type for now.

Note that each storage instance needs its own `#[pallet::storage]` attribute.

## Conceptual

The key difference between a `StorageValue` and a `StorageMap` is:

- A `StorageValue` stores a single value into a single key in the Merkle Trie.
- A `StorageMap` stores multiple values under different storage keys, all into different places in the Merkle Trie.

Let's clarify further.

Rust has a type `BTreeMap`, which is also a key/value map.

So what would be the difference between:

```rust
#[pallet::storage]
pub(super) type MyValueMap<T: Config> = StorageValue<Value = BTreeMap<u8, ()>>;
```

and

```rust
#[pallet::storage]
pub(super) type MyMap<T: Config> = StorageMap<Key = u8, Value = ()>;
```

They both can store the same data, but the `StorageValue` puts all of the data into a single object and stores that all into a single key in the Merkle Trie.

This means if we want to read just a single key / value pair, we must read ALL data in the whole map, and parse out just the single value we want.

In a `StorageMap`, each value is stored in its own spot in the Merkle Trie, so you are able to read just one key / value on its own. This can be way more efficient for reading just a single item.

However, trying to read multiple items from a `StorageMap` is extremely expensive.

So there is no perfect kind of storage, just tradeoffs.

## Use Cases

`StorageMap`s are really great when we need to store some unique information about a bunch of different things.

The most common example would be trying to store the token balance of all users in your blockchain. In this case, each user has their own `T::AccountId`, and that maps to some balance amount.

In our pallet, we use the `StorageMap` to store unique information about each `Kitty` in our pallet.

These use cases make sense because all the logic in our pallet typically touches only one key at a time.

- when you mint a kitty, we create one key / value.
- when you transfer a kitty, we mutate one key / value.
- when you put your kitty for sale, you mutate one key / value.
- etc...

And with the `StorageMap`, we can store a nearly infinite number of different kitties, or at least as many as there are unique keys.

## Your Turn

Add the `Kitties` storage map to your project as shown in the template.
