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

### Parity SCALE Codec

Parity SCALE Codec is a custom encoding and decoding library used in the `polkadot-sdk`.

The first question we are always asked when talking about SCALE, is why don't we use `<your favorite encoder>` instead?

Well, SCALE is:

- Simple to define.
- Not Rust-specific (but happens to work great in Rust).
	- Easy to derive codec logic: `#[derive(Encode, Decode)]`
	- Viable and useful for APIs like: `MaxEncodedLen` and `TypeInfo`
	- It does not use Rust `std`, and thus can compile to Wasm `no_std`.
- Consensus critical / bijective; one value will always encode to one blob and that blob will only decode to that value.
- Supports a copy-free decode for basic types on LE (Little-Endian) architectures (like Wasm).
- It is about as thin and lightweight as can be.

What you need to know about SCALE is that it defines how every object in the `polkadot-sdk` is represented in bytes.

### Max Encoded Length

Now that we have the tools to define the way objects should be encoded, we are able to create a trait which tracks the maximum encoded length of an object: `MaxEncodedLen`.

We then use that information to predict in the worst case scenario how much data will be used when we store it.

- For a `u8`, the `max_encoded_len()` is always the same: 1 byte.
- For a `u64`, the `max_encoded_len()` is always the same: 8 bytes.
- For a basic `enum`, it is also just 1 byte, since an enum can represent up to 256 variants.
- For a `struct`, the `max_encoded_len()` will be the sum of the `max_encoded_len()` of all items in the `struct`.

We need to be able to predict the size of items in our storage because it affects nearly all the constraints of our blockchain: storage size, memory usage, network bandwidth, and even execution time for encoding and decoding.

### Type Info

The last required trait for any storage item is `TypeInfo`.

This trait is key for off-chain interactions with your blockchain. It is used to generate metadata for all the objects and types in your blockchain.

Metadata exposes all the details of your blockchain to the outside world, allowing us to dynamically construct APIs to interact with the blockchain. This is super relevant since the `polkadot-sdk` is a framework for modular and upgradable blockchains.

We won't really use this in this tutorial, but it is super relevant to learn about once you start getting ready to actually use your blockchain.

**Skip Type Params**

One nasty thing about the `TypeInfo` derive macro, is that it isn't very "smart".

As I mentioned, the whole point of `TypeInfo` is to generate relevant metadata about the types used in your blockchain. However, part of our `Kitty` type is the generic parameter `T`, and it really does not make any sense to generate `TypeInfo` for `T`.

To make `TypeInfo` work while we have `T`, we need to include the additional line:

```rust
#[scale_info(skip_type_params(T))]
```

This tells the `TypeInfo` derive macro to simply "skip" the `T` type parameter when generating its code. The best thing for you to do is try compiling your code without this additional line, look at the errors that are generated, then see them disappear with the `skip_type_params`.

Then in the future, if you run into this error again, you will know what to do.

## Your Turn

Now that you know all about the various traits required for runtime development, derive them on the `Kitty` struct.

Don't forget to include the `skip_type_params(T)`.

After that, update your `Kitties` map to use `Value = Kitty<T>`.

Finally, update the logic in `mint` to create and insert this object into storage.

## Learn More

To get a primer on Parity SCALE Codec, check out this video from the Polkadot Blockchain Academy:

<iframe width="560" height="315" src="https://www.youtube.com/embed/6N6BopyYKq4?si=y7L9rmAoWbD803LW" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
