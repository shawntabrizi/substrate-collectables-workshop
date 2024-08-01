# Native Balance Type

One of the most challenging parts of using the `polkadot-sdk` is using generic types.

Hopefully, things like `T::AccountId` have been easy enough to use, but using the `Balance` type coming from `NativeBalance` can be a little tricky.

## Generic Types

The ability to use generic types in Rust is extremely powerful, but it can be hard to easily understand for those new to the `polkadot-sdk`. Not to mention that `polkadot-sdk` uses a lot of generic types.

The key thing to remember is that all of these generic types eventually become a concrete type. So while we are not sure while we write our pallet if `T::AccountId` is `[u8; 20]`, `[u8; 32]`, or maybe even a `String`, we know that eventually it must be one of these primitive types.

In this situation, the same can be said for the `Balance` type coming from `NativeBalance`. Depending on the configuration of your blockchain, and the required traits that `Balance` needs to satisfy, it is perfectly valid for your `Balance` type to be `u32`, `u64`, or `u128`.

But it can only concretely be one of those, and the weird thing is that we don't know which one it is as we program our Kitties pallet!

Let's look at how we would interact with this generic type, and solve many of the issues you might encounter when trying to use it.

## Balance Type

The `Balance` type is ultimately configured inside `pallet_balances`, and remember, we don't have direct access to that pallet because we used loose coupling.

The way we can access the `Balance` type is through the `Inspect` trait of the `NativeBalance` associated type. Accessing it kind of funny, which is why we commonly introduce a `BalanceOf<T>` alias type like so:

```rust
// Allows easy access our Pallet's `Balance` type. Comes from `Fungible` interface.
pub type BalanceOf<T> =
	<<T as Config>::NativeBalance as Inspect<<T as frame_system::Config>::AccountId>>::Balance;

```

This is kind of a crazy line of code, so let's break it down:

- At the very end, there is a `Balance` type. This is what we want to access and alias.
	```rust
	Balance
	```
- This `Balance` type is part of the `Inspect` trait.
	```rust
	Inspect::Balance
	```
- The `Inspect` trait is generic over `AccountId`, so we need to include that.
	```rust
	Inspect<AccountId>::Balance
	```
- The `AccountId` type comes from `T`, through `frame_system::Config`, where the type is defined.
	```rust
	Inspect<<T as frame_system::Config>::AccountId>::Balance
	```
- The `Inspect` is accessible through the `NativeBalance` associated type.
	```rust
	<NativeBalance as Inspect<<T as frame_system::Config>::AccountId>>::Balance
	```
- The `NativeBalance` type also comes from `T`, but though our pallet's own `Config`.
	```rust
	<<T as Config>::NativeBalance as Inspect<<T as frame_system::Config>::AccountId>>::Balance
	```
- Finally, we assign this to a new alias `BalanceOf` which is generic over `<T>`
	```rust
	pub type BalanceOf<T> = <<T as Config>::NativeBalance as Inspect<<T as frame_system::Config>::AccountId>>::Balance
	```

Phew! Did you get all that? If not, don't worry too much. You can review these Rust concepts after you complete the tutorial, but there is nothing here which is specific to the `polkadot-sdk`.

Why do we need this `BalanceOf<T>` alias?

So that we can change this:

```rust
fn set_price(id: [u8; 16], price: <<T as Config>::NativeBalance as Inspect<<T as frame_system::Config>::AccountId>>::Balance) {
	// -- snip --
}
```

To this:

```rust
fn set_price(id: [u8; 16], price: BalanceOf<T>) {
	// -- snip --
}
```

The second is way better right? This type alias handles extracting the `Balance` type out of the `NativeBalance` associated type every time, and all we need to do is pass the generic parameter `T`.

## Basic API

- TODO:
	- Introduce the `BalanceOf<T>` type
		- explain how it navigates the traits to access the underlying type
		- explain how it is just a number, but can be any type of number
		- explain `AtLeast32BitsUnsigned`
	- Show how to use the `BalanceOf<T>` type.
