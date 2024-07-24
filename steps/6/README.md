# Pallet Config

Each pallet includes a trait `Config` which is used to configure the pallet in the context of your larger runtime.

```rust
#[pallet::config]
pub trait Config: frame_system::Config {
	// -- snip --
}
```

It sucks to keep repeating this about different parts of FRAME development, but the full power of the `Config` trait can only be understood once you have moved passed the basics.

For now, we just want to focus on the basics.

## T as Config

We use our Pallet's `Config` all over our code, but through a generic parameter `T`.

This is what is meant with `<T: Config>` that you see everywhere.

The simplest way to understand is that wherever you see `T`, you have access to our `trait Config` and the types and functions inside of it.

## Trait Inheritance

To understand how we use the `Config` trait, we first need to learn about basic Rust inheritance patterns.

```rust
example here i think
```

You will notice that our `Config` trait inherits from the `frame_system::Config` trait.

What is `frame_system`? What is in it's config?

## FRAME System

In order for our blockchain to function, we need some base level primitives.

- Account Id
- Block Number
- Block Hash
- Nonce
- etc...

`frame_system` provides all of that, and all the basic level functions needed for your blockchain to operate.

These types, and more, are defined within the `frame_system::Config`:

```rust
pub trait Config: 'static + Eq + Clone {
	type Hash: Parameter + Member + MaybeSerializeDeserialize + Debug + MaybeDisplay + SimpleBitOps + Ord + Default + Copy + CheckEqual + sp_std::hash::Hash + AsRef<[u8]> + AsMut<[u8]> + MaxEncodedLen;
	type AccountId: Parameter + Member + MaybeSerializeDeserialize + Debug + MaybeDisplay + Ord + MaxEncodedLen;
	type Block: Parameter + Member + traits::Block<Hash = Self::Hash>;
	type Nonce: Parameter + Member + MaybeSerializeDeserialize + Debug + Default + MaybeDisplay + AtLeast32Bit + Copy + MaxEncodedLen;
	// -- snip --
}
```

Because our `trait Config` inherits from `frame_system::Config`, we have access to these types too.

This is why you see in our starting code `T::AccountId`. We are able to access the `AccountId` type, which is originally defined inside `frame_system::Config` through our `trait Config`, via the generic trait `T`.

Phew.

If this doesn't make sense, that's okay. You should be able to follow the patterns for successfully programming all this, and you can learn the deep Rust stuff later.

## Our Config

Our config only includes one item for now: `RuntimeEvent`.

It has a pretty nasty trait bound:

```rust
type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
```

The main purpose of this trait bound is to allow events of this pallet to be converted to and from a

- TODO: explain this stuff
