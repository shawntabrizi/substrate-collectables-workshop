Buying a Kitty
===

Now that we can set the price for a kitty, and we can transfer ownership of a kitty, we have everything we need to build a `buy_kitty` function.

## Check a Kitty is for Sale

Before we allow a user to execute the `buy_kitty()` function, we shold make sure tha the kitty is indeed for sale. We have simplified our example such that any Kitty with the default price of 0 is not for sale. Owners can easily call `set_price()` on their kitty with the value of 0, and take it off the market.

If you wanted to improve this, we may have the price be an `Option<T::Balance>`, where 0 would be a valid price, and not for sale would be represented by `None`... but we will leave that as a challenge for the reader.

## Checking a Balance `is_zero()`

Types like `T::Balance` have traits which ensure that this type has support for features and functionality that you can consistantly use.

[TODO: Is it worth introducing this in our tutorial? Should we simplify or just glaze over this?]

## Making a Payment

[TODO: Explain Balances module, free balance, reserved balance, etc...]

## Your Turn!