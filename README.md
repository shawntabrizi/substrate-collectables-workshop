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

## Supertraits

To understand how we use the `Config` trait, we first need to learn about [Rust supertraits](https://doc.rust-lang.org/rust-by-example/trait/supertraits.html).

Supertraits are similar to the concept of "inheritance" from other programming languages. In Rust, it allows one trait as being a superset of another trait.

You will notice that our `Config` trait is a subtrait of the supertrait `frame_system::Config`.

What is `frame_system`? What is in `frame_system::Config`?

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

Because our `trait Config` is a superset of `frame_system::Config`, we have access to these types too.

This is why you see in our starting code `T::AccountId`. We are able to access the `AccountId` type, which is originally defined inside `frame_system::Config` through our `trait Config`, via the generic trait `T`.

Phew.

If this doesn't make sense, that's okay. You should be able to follow the patterns for successfully programming all this, and you can learn the deep Rust stuff later.

## Our Config

Our config only includes one item for now: `RuntimeEvent`.

It has a pretty nasty trait bound:

```rust
type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
```

The main purpose of this trait bound is to allow events of this pallet to be converted to and from an "aggregated" event type, which contains all possible event variants from all possible Pallets in our blockchain.

Remember, our runtime is composed of multiple pallets, some we create, some which come with the `polkadot-sdk`, some that we import from 3rd parties.

Each of these pallets will want to include their own custom events, and our blockchain as a whole needs to be able to handle all of them.

The `RuntimeEvent` type, with the help of our macros, aggregates all of these events coming from all of these pallets. These trait bounds help us use this type!

If you want to learn more about this (super optional), check out this video:

<iframe width="560" height="315" src="https://www.youtube.com/embed/OCBC1pMYPoc?si=hFBq42GN_q_Eo0zs" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
