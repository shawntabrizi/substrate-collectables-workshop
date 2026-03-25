# Pallet Config

Each pallet includes a trait `Config` which is used to configure the pallet in the context of your larger runtime.

```rust
#[pallet::config]
pub trait Config: frame_system::Config {
	// -- snip --
}
```

It sucks to keep repeating this about different parts of FRAME development, but the full power of the `Config` trait can only be understood once you have passed the basics.

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

Our config is currently empty:

```rust
#[pallet::config]
pub trait Config: frame_system::Config {}
```

Even though our `Config` is empty, there is still important work happening behind the scenes. The FRAME macros automatically generate a `RuntimeEvent` type for our pallet, which aggregates all events from all pallets in the runtime into a single enum. This is what allows our pallet's events to be deposited into the system and accessed by tools like block explorers and indexers.

In older versions of the Polkadot SDK, you would need to explicitly define a `RuntimeEvent` associated type in your pallet's `Config`. This is no longer necessary — the FRAME macros handle this for you automatically.

As we build out our pallet, we will add new associated types to our `Config` trait when we need to access external functionality, like the Balances Pallet.

If you want to learn more about this (super optional), check out this video:

<iframe width="560" height="315" src="https://www.youtube.com/embed/OCBC1pMYPoc?si=hFBq42GN_q_Eo0zs" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
