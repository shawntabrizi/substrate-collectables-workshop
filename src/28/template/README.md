# Set Price Extrinsic

Now that we our Pallet set up to handle balances, let's actually use them.

In this step, we will create an extrinsic which allows the owner of a kitty to set a price for the kitty.

## Set Price

The set price extrinsic is pretty straight forward.

As we noted in the last step, we allow the `price` field of a kitty to be `Option<BalanceOf<T>>`, where `None` denotes a kitty which is not for sale, and `Some(price)` denotes a kitty which is for sale at some `price`.

With this in mind, our `set_price` extrinsic will also accept an `Option<BalanceOf<T>>` so that a user can put the kitty on or off the market.

## Your Turn

In this step, you will scaffold the extrinsic and internal functions for `set_price`.

- Create a new event `PriceSet` with the fields noted in the template.
- Create a new extrinsic `set_price` with the params noted in the template.
- Create a new internal function `do_set_price` which simply deposits the `PriceSet` event.

We will actually populate the logic for `do_set_price` in the next step.
