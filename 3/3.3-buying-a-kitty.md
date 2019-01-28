Buying a Kitty
===

Now that we can set the price for a kitty and transfer ownership of a kitty, we have everything we need to build a `buy_kitty` function.

## Check a Kitty is for Sale

Before we allow a user to execute the `buy_kitty()` function, we should make sure that the kitty is indeed for sale. We have simplified our example such that any Kitty with the default price of 0 is not for sale. Owners can easily call `set_price()` on their kitty with the value of 0, and take it off the market.

You can easily check that a `T::Balance` is zero using a function exposed by the type:

```rust
let my_value = <T::Balance as As<u64>>::sa(0);
ensure!(my_value.is_zero(), "Value is not zero");
// `ensure` will succeed and execution continues here
```

If you wanted to improve this, we may have the price be an `Option<T::Balance>`, where 0 would be a valid price, and not for sale would be represented by `None`... but we will leave that as a challenge for the reader.

## Making a Payment

So far our chain has been completely independent of our internal currency provided by the `Balances` module. The `Balances` module gives us access to completely manage the internal currency of every user, which means we need to be careful how we use it.

Fortunately, the `Balances` module exposes a public function called `make_transfer()` which allows you to safely transfer units from one account to another, checking for enough balance, overflow, underflow, and even account creation as a result of getting tokens.

This function both "verifies" and "writes", so you will need to be careful exactly where you include it as a part of your module's logic.

```rust
// end of verifications

<balances::Module<T>>::make_transfer(&to, &from, value)?;

// beginning of writing to storage
```

## Your Turn!

Follow the template provided to program in the necessary code to complete the `buy_kitty()` function. Feel free to test your new function in the Polkadot-JS Apps UI for any bugs.

[embedded-code](./assets/3.3-template.rs ':include :type=code embed-template')

<a href="javascript:toggleHint()" id="hint_link">Reveal the solution...</a>

[embedded-code-final](./assets/3.3-finished-code.rs ':include :type=code embed-final')