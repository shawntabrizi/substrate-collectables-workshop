# Safety First

If you look into the history of "hacks" and "bugs" that happen in the blockchain world, a lot of it is associated with some kind of "unsafe" code.

Keeping our blockchain logic safe, and Rust is designed to handle it well.

## Errors

When talking about handling safe math, we will start to introduce and use errors.

### Do Not Panic!

If there is only one thing you remember after this whole tutorial, it should be this fact:

**You cannot panic inside the runtime.**

As a runtime developer, you are building logic in the low level parts of your blockchain.

A smart contract system must be able to handle malicious developers, but this comes at a performance cost.

When you program directly in the runtime, you get the highest performance possible, but you are also expected to be a competent developer and a good actor.

In short, if you introduce a panic in your code, you make your blockchain vulnerable to DDoS attacks.

But there is no reason you would ever need to panic because Rust has a great error handling system that we take advantage of in FRAME.

### Pallet Errors

All of our callable functions use the `DispatchResult` type. This means that we can always propagate up any errors that our Pallet runs into, and handle them properly, versus needing to panic.

The [`DispatchResult`](https://docs.rs/frame-support/38.0.0/frame_support/dispatch/type.DispatchResult.html) type expects either `Ok(())` or `Err(DispatchError)`.

The [`DispatchError`](https://docs.rs/frame-support/38.0.0/frame_support/pallet_prelude/enum.DispatchError.html) type has a few variants that you can easily construct / use.

For example, if you want to be a little lazy, you can simply return a `&'static str`:

```rust
fn always_error() -> DispatchResult {
	return Err("this function always errors".into())
}
```

But the better option is to return a custom Pallet Error:

```rust
fn custom_error() -> DispatchResult {
	return Err(Error::<T>::CustomPalletError.into())
}
```

Notice in both of these cases we had to call `into()` to convert our input type into the `DispatchError` type.

To create `CustomPalletError` or whatever error you want, you simply add a new variants to the `enum Error<T>` type.

```rust
#[pallet::error]
pub enum Error<T> {
	/// This is a description for the error.
	///
	/// This description can be shown to the user in UIs, so make it descriptive.
	CustomPalletError,
}
```

We will show you the common ergonomic ways to use Pallet Errors going forward.

## Math

### Unsafe Math

The basic math operators in Rust are **unsafe**.

Imagine our `CountForKitties` was already at the limit of `u32::MAX`. What would happen if we tried to call `mint` one more time?

We would get an overflow!

In tests `u32::MAX + 1` will actually trigger a panic, but in a `release` build, this overflow will happen silently...

And this would be really bad. Now our count would be back to 0, and if we had any logic which depended on this count being accurate, that logic would be broken.

In blockchain systems, these can literally be billion dollar bugs, so let's look at how we can do math safely.

### Checked Math

The first choice for doing safe math is to use `checked_*` APIs, for example [`checked_add`](https://docs.rs/num/latest/num/trait.CheckedAdd.html).

The checked math APIs will check if there are any underflows or overflows, and return `None` in those cases. Otherwise, if the math operation is calculated without error, it returns `Some(result)`.

Here is a verbose way you could handle checked math in a Pallet:

```rust
let final_result: u32 = match value_a.checked_add(value_b) {
	Some(result) => result,
	None => return Err(Error::<T>::CustomPalletError.into()),
};
```

You can see how we can directly assign the `u32` value to `final_result`, otherwise it will return an error.

We can also do this as a one-liner, which is more ergonomic and preferred:

```rust
let final_result: u32 = value_a.checked_add(value_b).ok_or(Error::<T>::CustomPalletError)?;
```

This is exactly how you should be writing all the safe math inside your Pallet.

Note that we didn't need to call `.into()` in this case, because `?` already does this!

### Saturating Math

The other option for safe math is to use `saturating_*` APIs, for example [`saturating_add`](https://docs.rs/num/latest/num/traits/trait.SaturatingAdd.html).

This option is useful because it is safe and does NOT return an `Option`.

Instead, it performs the math operations and keeps the value at the numerical limits, rather than overflowing. For example:

```rust
let value_a: u32 = 1;
let value_b: u32 = u32::MAX;
let result: u32 = value_a.saturating_add(value_b);
assert!(result == u32::MAX);
```

This generally is NOT the preferred API to use because usually you want to handle situations where an overflow would occur. Overflows and underflows usually indicate something "bad" is happening.

However, there are times where you need to do math inside of functions where you cannot return a Result, and for that, saturating math might make sense.

There are also times where you might want to perform the operation no matter that an underflow / overflow would occur. For example, imagine you made a function `slash` which slashes the balance of a malicious user. Your slash function may have some input parameter `amount` which says how much we should slash from the user.

In a situation like this, it would make sense to use `saturating_sub` because we definitely want to slash as much as we can, even if we intended to slash more. The alternative would be returning an error, and not slashing anything!

Anyway, every bone in your body should generally prefer to use the `checked_*` APIs, and handle all errors explicitly, but this is yet another tool in your pocket when it makes sense to use it.

## Your Turn

We covered a lot in this section, but the concepts here are super important.

Feel free to read this section again right now, and again at the end of the tutorial.

Now that you know how to ergonomically do safe math, update your Pallet to handle the `mint` logic safely and return a custom Pallet Error if an overflow would occur.
