# Value Query

When we originally introduced the `StorageValue`, we only exposed to you one field which you could manipulate, which is the `Value` type, which defines the type that will be stored.

But there are actually more ways you can configure the `StorageValue` to increase developer ergonomics.

## Storage Value Definition

Let's look at the definition of a [`StorageValue`](https://docs.rs/frame-support/37.0.0/frame_support/storage/types/struct.StorageValue.html):

```rust
pub struct StorageValue<
	Prefix,
	Value,
	QueryKind = OptionQuery,
	OnEmpty = GetDefault
>(_);
```

You can see the `StorageValue` expects 4 different generic parameters, two of which have default implementations, which means you don't need to define them unless you want to change the configuration.

### Prefix

We have been able to hide the `Prefix` type from you so far because the `#[pallet::storage]` macro implements this for us auto-magically.

We won't go into detail about what exactly `Prefix` does, but the high level idea is that it contains metadata needed to be able to correctly, uniquely, and consistently read and write data into the merkle trie.

Those details are pretty low level, and something we can automatically implement for you with the macros. You should never need to do anything with `Prefix`, else you are probably doing something wrong.

### Value

You already know how to use and implement `Value` for `StorageValue`, so we won't go into too much detail here.

I will note that not literally every object can be placed in storage. It must satisfy some traits, which we will learn later in the tutorial.

### Query Kind

You can see the `QueryKind` parameter defaults to `OptionQuery`.

This is why when we query storage, we get an `Option` returned to us.

Now as we stated before, Runtime storage is ALWAYS an `Option`. It is always possible that some storage does not exist / is not set when we first try to query it. But as we showed in our Pallet so far, there is logic we can write to "hide" that fact from the user, like `unwrap_or(0)`.

We can do this even more ergonomically by changing the `QueryKind` to `ValueQuery`.

```rust
#[pallet::storage]
pub(super) type CountForKitties<T: Config> = StorageValue<Value = u32, QueryKind = ValueQuery>;
```

In this case, all of our APIs change.

Now when you call `get`, you will directly return a value, and when you call `set`, you just need to pass the value, not an option.

Now you might ask: "What value do you get when you call `get` and there is no value stored?".

That is controlled by the `OnEmpty` type.

### On Empty

As we noted above, the `OnEmpty` type defines the behavior of the `QueryKind` type.

When you call `get`, and the storage is empty, the `OnEmpty` configuration kicks in.

The default configuration for `OnEmpty` is `GetDefault`. This of course requires that the `Value` type must implement `Default`. But if it does, then you will get the following behavior:

```rust
assert!(CountForKitties::<T>::get() == u32::default());
```

For numbers, this value is normally zero, so simply setting `QueryKind = ValueQuery` gives you exactly the same behavior as what we programmed in our Pallet so far.

If your type does not implement `Default`, you can't use `ValueQuery`. A common example of this is the `T::AccountId` type, which purposefully has no default value, and thus is not compatible out of the box with `ValueQuery`.

You CAN modify `OnEmpty` to return a custom value, rather than `Default`, but we won't cover that here. Feel free to explore this idea on your own.

## Your Turn

Update your `CountForKitties` to use `QueryKind = ValueQuery`.

This will affect the APIs for `get` and `set`, so also update your code to reflect those changes.
