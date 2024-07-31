# Storing a Struct

We have successfully created a generic struct for our pallet. Now we need to actually use it in our runtime.

## Derive Macros

One of the powerful tools you get from vanilla Rust is the `#[derive(...)]` macros.

In the spirit of Rust macros, derive macros help reduce boiler plate code which can be automatically generated for you. In this case, the derive macros generate trait implementations for the objects they are applied on.

The most simple example might be `Default`.

The verbose way of implementing `Default` would be:

```rust
pub struct MyObject {
	field_0: u32,
	field_1: u64,
}

impl Default for MyObject {
	fn default() -> Self {
		Self {
			field_0: Default::default(),
			field_1: Default::default(),
		}
	}
}
```

You can see here that we can simply say the default for `MyObject` is taking the default of each field in `MyObject` and constructing the struct.

We can do the exact same thing with `#[derive(Default)]`:

```rust
#[derive(Default)]
pub struct MyObject {
	field_0: u32,
	field_1: u64,
}
```

As long as all the fields inside `MyObject` implement `Default`, then the derive macro will handle all the magic.

> Remember that `T::AccountId` explicitly chooses not to implement `Default`, so you cannot implement `Default` on the `Kitty` struct.

## Traits Required for Storage

For an object to be placed inside runtime storage, we require it to have a number of traits implemented:

- `Encode`: The object must be encodable to bytes using `parity_scale_codec`.
- `Decode`: The object must be decodable from bytes using `parity_scale_codec`.
- `MaxEncodedLen`: When the object is encoded, it must have an upper limit to its size.
- `TypeInfo`: The object must be able to generate metadata describing the object.

All of these things are pretty specific to the requirements of using the Polkadot-SDK for building a blockchain.

### Parity Scale Codec

TODO: quick details about codec, link to docs

### Max Encoded Len

TODO: Talk about limits with "memory"

### Type Info

TODO: Talk about metadata

#### Skip Type Params

## Your Turn

Now that you know all about the various traits required for runtime development, derive them on the `Kitty` struct.

Don't forget to include the `skip_type_params(T)`.

After that, update your `Kitties` map to use `Value = Kitty<T>`.

Finally, update the logic in `mint` to create and insert this object into storage.
