# Origin

As we started to describe, the `origin` is the first parameter of every callable function.

It describes where the call is calling from, and allows us to perform simple access control logic based on that information.

## Origin vs Sender

If you are familiar with smart contract development, for example in Ethereum, you will be familiar with `msg.sender`.

Origin is a superset of this idea. No longer do we need to assume that every call to a callable function is coming from an external account. We could have pallets call one another, or other internal logic trigger a callable function.

It is hard to explain the power of Origin when you are still learning the basics of Pallet development, but this is again, something worth exploring deeper at a later point.

## Ensure Signed

In this tutorial, we will just use origin to check for signed messages.

For this, we can use the `ensure_signed` function:

```rust
let who: T::AccountId = ensure_signed(origin)?;
```

You can see this function takes the `OriginFor<T>` type, and will return a `T::AccountId` if the `origin` was an account, otherwise it will return the error `BadOrigin`.

This turns origin into exactly the same as `msg.sender` from Ethereum contract development.

With this, we are able to

## Deep Dive

As a note, you should know that `ensure_signed` is not actually doing signature checking.

Signature checking is very expensive, perhaps one of the most expensive things to perform when executing transactions.

So signature checking happens batched and parallelized at the beginning of executing a block.

By the time your callable function gets the `origin`, it is just:

```rust
let origin: OriginFor<T> = RawOrigin::Signed(account_id).into();
```

So it is simply an enum variant with the `T::AccountId` inside. So `ensure_signed` logic is as simple as:

```rust
pub fn ensure_signed(o: OriginFor<T>) -> Result<AccountId, BadOrigin> {
	match o {
		RawOrigin::Signed(t) => Ok(t),
		_ => Err(BadOrigin),
	}
}
```

The real `ensure_signed` function has more generic stuff, but the idea is the same.
