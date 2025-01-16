# Duplicate Kitty Check

To make sure your state transition function behaves as expected, you must check everything that could go wrong, and return an error in those cases.

## Contains Key

As we mentioned in the previous section, the `insert` API will simply overwrite any existing data which is already there. We want to prevent this, else we risk overwriting data about a kitty that already exists and is already owned by someone.

To prevent this, we can call the `contains_key(key)` API, which will return `true` if there is already a value in storage, or `false` if there isn't.

You might ask: Why do we call `contains_key` rather than call `get` and checking the `Option`?

Well, there are two reasons:

1. Just like `StorageValue`, you can set a `StorageMap` with `QueryKind = ValueQuery`, thus we would no longer return an `Option` for you to check, and we would "hide" the fact that the map is empty at that key. So this API is the only way to check if the value ACTUALLY exists or not.
2. The `contains_key` API is strictly more efficient than the `get` API. The `get` API returns the value, which means we need to go into the database, read the bits, serialize it to a Rust type, pass that back to the Pallet, and assign it to the variable.

	On the other hand, `contains_key` can simply check if the value exists in the database, and only return back the boolean result. No need to serialize all the bytes, store the variable, or any of that.

This same trick applies to `StorageValue` too. Rather than calling `get`, you can call `exists`, which provides the same api as `contains_key`.

## Ensure

To do simple duplication checks and return an error, we can write the following:

```rust
if (PalletKitties::<T>::contains_key(my_key)) {
	return Err(Error::<T>::DuplicateKitty.into());
}
```

But that is pretty verbose. We have a macro which can do this for you in a single line:

```rust
ensure!(!Kitties::<T>::contains_key(my_key), Error::<T>::DuplicateKitty);
```

`ensure!` is a macro, which basically expands into the same verbose code shown above, except checking the opposite condition. That is to say, we `ensure!` that `Kitties` does NOT `contains_key`, else we return the error specified.

There really is no difference here, so use whatever makes you comfortable.

## Your Turn

Now that you know how to efficiently check for an existing kitty, put that check into the `mint` function to ensure that we do not mint a duplicate kitty in the `Kitties` storage map.
