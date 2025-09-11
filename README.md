# Bounded Vectors

We placed a vector in storage in the last step. This is okay for initial development, but this is NOT okay for a production Pallet. Instead we need to use objects which have a `MaxEncodedLen`, and for that, we have the `BoundedVec` type.

## Max Encoded Length

We mentioned earlier that we require all blockchain storage to have a maximum upper limit to the encoded length of that object. For that, we use the `MaxEncodedLen` trait.

But what is the `max_encoded_len()` of a `Vec<T>`?

One answer might be: approximately `T * 2^32`; but this is not a reasonable answer. :)

In fact, we do not implement `MaxEncodedLen` on `Vec` because the answer is so unreasonable.

So we need to create a new structure which can act like a `Vec`, but also have reasonable bounds as to how many items are inside of it.

Hence the `BoundedVec` was born.

## Construction

The `BoundedVec` type is a zero-overhead abstraction over the `Vec` type allowing us to control the maximum number of item in the vector.

To create a new `BoundedVec` with a maximum of 100 `u8`s, you can do the following:

```rust
let my_bounded_vec = BoundedVec::<u8, ConstU32<100>>::new();
```

The syntax here is very similar to creating a `Vec`, however we include a second generic parameter which tells us the bound. The easiest way to set this bound is using the `ConstU32<T>` type.

There are other ways to define the bound, and even make it configurable, but that is beyond the scope of this tutorial. Add it to the list of things to follow up on after you have completed this tutorial.

## Basic APIs

The `BoundedVec` type has almost all the same APIs as a `Vec`. You can find the full list of APIs in the [`BoundedVec` documentation](https://docs.rs/frame-support/38.0.0/frame_support/struct.BoundedVec.html).

The main difference is the fact that a `BoundedVec` cannot always accept a new item.

So rather than having `push`, `append`, `extend`, `insert`, and so on, you have `try_push`, `try_append`, `try_extend`, `try_insert`, etc...

These functions have the same parameters as their `Vec` equivalent, but can return a `Result` rather than being infallible.

So converting the logic of a `Vec` to a `BoundedVec` can be as easy as:

```rust
// Append to a normal vec.
vec.append(item);
// Try append to a bounded vec, handling the error.
bounded_vec.try_append(item).map_err(|_| Error::<T>::TooManyOwned)?;
```

## Storage Optimizations

Just like for `Vec`, our `BoundedVec` also has an optimized `try_append` API for trying to append a new item to the `BoundedVec` without having to read the whole vector in the runtime.

The change to use this API also looks pretty much the same as above:

```rust
// Append to a normal vec.
KittiesOwned::<T>::append(item);
// Try append to a bounded vec, handling the error.
KittiesOwned::<T>::try_append(item).map_err(|_| Error::<T>::TooManyOwned)?;
```

## Your Turn

Update the `KittiesOwned` storage map to use `Value = BoundedVec` with up to 100 items.

You will need to update the logic for the `mint` function to handle the case where we cannot mint a new kitty for an `owner`. For that, we will need to introduce a new error `TooManyOwned`.
