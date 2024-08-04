# Kitty Counter

Let's now learn how to use our new `StorageValue`.

## Basic APIs

This tutorial will only go over just the basic APIs needed to build our Pallet.

Check out the [`StorageValue` documentation](https://docs.rs/frame-support/37.0.0/frame_support/storage/types/struct.StorageValue.html) if you want to see the full APIs.

### Reading Storage

To read the current value of a `StorageValue`, you can simply call the `get` API:

```rust
let maybe_count: Option<u64> = CountForKitties::<T>::get();
```

A few things to note here.

The most obvious one is that `get` returns an `Option`, rather than the type itself.

In fact, all storage in a blockchain is an `Option`: either there is some data in the database or there isn't.

In this context, when there is no value in storage for the `CountForKitties`, we probably mean that the `CountForKitties` is zero.

So we can write the following to handle this ergonomically:

```rust
let count: u64 = CountForKitties::<T>::get().unwrap_or(0);
```

Now, whenever `CountForKitties` returns `Some(count)`, we will simply unwrap that count and directly access the `u64`. If it returns `None`, we will simply return `0u64` instead.

The other thing to note is the generic `<T>` that we need to include. You better get used to this, we will be using `<T>` everywhere! But remember, in our definition of `CountForKitties`, it was a type generic over `<T: Config>`, and thus we need to include `<T>` to access any of the APIs.

### Writing Storage

To set the current value of a `StorageValue`, you can simply call the `set` API:

```rust
CountForKitties::<T>::set(Some(1u64));
```

This storage API cannot fail, so there is no error handling needed. You just set the value directly in storage. Note that `set` will also happily replace any existing value there, so you will need to use other APIs like `exists` or `get` to check if a value is already in storage.

If you `set` the storage to `None`, it is the same as deleting the storage item.

## Your Turn

Now that you know the basics of reading and writing to storage, add the logic needed to increment the `CountForKitties` storage whenever we call `mint`.
