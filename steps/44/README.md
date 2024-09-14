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
- Finally, we assign this to a new alias `BalanceOf` which is generic over `<T>`.
	```rust
	pub type BalanceOf<T> = <<T as Config>::NativeBalance as Inspect<<T as frame_system::Config>::AccountId>>::Balance
	```

Phew! Did you get all that? If not, don't worry too much. You can review these Rust concepts after you complete the tutorial, but there is nothing here which is specific to the `polkadot-sdk`.

Why do we need this `BalanceOf<T>` alias?

So that we can change this:

```rust
fn set_price(id: [u8; 32], price: <<T as Config>::NativeBalance as Inspect<<T as frame_system::Config>::AccountId>>::Balance) {
	// -- snip --
}
```

To this:

```rust
fn set_price(id: [u8; 32], price: BalanceOf<T>) {
	// -- snip --
}
```

The second is way better right? This type alias handles extracting the `Balance` type out of the `NativeBalance` associated type every time, and all we need to do is pass the generic parameter `T`.

## Basic API

Let's learn how we can use the `BalanceOf<T>` type in our code.

### Interacting with Primitive Numbers

I will repeat again: Because the `BalanceOf<T>` type is generic, we cannot know what underlying type it is. This means we CANNOT write the following:

```rust
// This code doesn't work
fn add_one(input: BalanceOf<T>) -> BalanceOf<T> {
	input + 1u128
}
```

Even if we don't include `u128`, we cannot write the line above. This is because that line assumes that `input` must be some specific number type, and in that code, it is simply generic.

However, `BalanceOf<T>` [does have traits](https://docs.rs/frame-support/37.0.0/frame_support/traits/tokens/trait.Balance.html) that we can use to interact with it. The key one being [`AtLeast32BitUnsigned`](https://docs.rs/polkadot-sdk-frame/0.6.0/polkadot_sdk_frame/arithmetic/trait.AtLeast32BitUnsigned.html).

This means our `BalanceOf<T>` must be an unsigned integer, and must be at least `u32`. So it could be `u32`, `u64`, `u128`, or even bigger if you import other crates with those larger unsigned types.

This also means we **would** be able to write the following:

```rust
// This code does work
fn add_one(input: BalanceOf<T>) -> BalanceOf<T> {
	input + 1u32.into()
}
```

We can convert any `u32` into the `BalanceOf<T>` type because we know at a minimum `BalanceOf<T>` is `AtLeast32BitUnsigned`.

### Interacting with Itself

Interacting between two `BalanceOf<T>` types will act just like two normal numbers of the same type.

You can add them, subtract them, multiply them, divide them, and even better, do safe math operations on all of them.

```rust
let total_balance: BalanceOf<T> = balance_1.checked_add(balance_2).ok_or(ArithmeticError::Overflow)?;
```

## Price Field

We are going to use `BalanceOf<T>` in the `Kitty` struct to keep track if it is for sale, and the price the owner wants.

For this we can use an `Option<BalanceOf<T>>`, where `None` denotes that a kitty is not for sale, and `Some(price)` denotes the kitty is for sale at some `price`.

### Resetting the Price Field

In this step, we are introducing a new `price` field to the `Kitty` struct, and we must consider how that might affect existing logic in our Pallet.

The price is something that should only be set by the owner of the kitty, and describes what the current owner would want to sell the kitty for.

However, when a kitty is transferred to a new owner, that new owner may not agree with the existing price or want to sell the kitty at all!

Thus, whenever we transfer the kitty, we will want to reset the `price` to `None` to make sure it is not immediately for sale.
The new owner will be able to set the new price if they want to sell their new kitty.

## Your Turn

Now that you know how to create and use the `BalanceOf<T>` type, add the type alias to your Pallet as shown in the template.

Then add a new field to the `Kitty` struct called `price`, which is an `Option<BalanceOf<T>>`.

Update the `mint` function to create a new `Kitty` with the new `price` field set as `None`.

Update the `do_transfer` function to reset the `kitty.price` to `None` when the kitty gets a new owner.

Finally, update your `tests.rs` file so that `DEFAULT_KITTY` has the field and value `price: None`.
