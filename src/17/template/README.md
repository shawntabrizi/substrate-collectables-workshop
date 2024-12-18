# Kitties Map

Now let's learn to interact with our `Kitties` storage map, and update the map when we `mint` new kitties.

## Basic APIs

This tutorial will only go over just the basic APIs needed to build our Pallet.

Check out the [`StorageMap` documentation](https://docs.rs/frame-support/38.0.0/frame_support/storage/types/struct.StorageMap.html) if you want to see the full APIs.

### Reading Storage

To read the current value of a key in a `StorageMap`, you can simply call the [`get(key)`](https://docs.rs/frame-support/38.0.0/frame_support/storage/types/struct.StorageMap.html#method.get) API:

```rust
let my_key: [u8; 32] = [0u8; 32];
let maybe_value: Option<()> = Kitties::<T>::get(my_key);
```

Just as the `StorageValue`, you can see this query returns an `Option`, indicating whether there is actually a value under the key.

Just as before, the most ergonomic way to read a kitty, or throw an error when there is no kitty is to write the following:

```rust
let kitty: () = Kitties::<T>::get(my_key).ok_or(Error::<T>::NoKitty)?;
```

### Writing Storage

To add a new value to the `StorageMap`, you can simply call the `insert` API:

```rust
let my_key: [u8; 32] = [0u8; 32];
Kitties::<T>::insert(my_key, ());
```

The same behaviors apply to `StorageMap` as a `StorageValue`.

The [`insert`](https://docs.rs/frame-support/38.0.0/frame_support/storage/types/struct.StorageMap.html#method.insert) API cannot fail. If a value already exists in the map, under the key, we will simply overwrite that value. If you want to check if a value already exists in the map under a key, the most efficient way is to call [`contains_key(key)`](https://docs.rs/frame-support/38.0.0/frame_support/storage/types/struct.StorageMap.html#method.contains_key).

## Your Turn

`StorageMap`s are easy!

Update the logic in your pallet to insert a new kitty into your `Kitties` map when we call `mint`.

For this, you will need to add a second parameter to the `mint` function to pass the unique identifier for the kitty.
