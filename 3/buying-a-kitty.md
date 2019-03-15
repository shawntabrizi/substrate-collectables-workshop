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

Fortunately, the `Balances` module exposes a public function called [`make_transfer()`](https://crates.parity.io/srml_balances/struct.Module.html#method.make_transfer) which allows you to safely transfer units from one account to another, checking for enough balance, overflow, underflow, and even account creation as a result of getting tokens.

## Remember: "Verify First, Write Last"

If you look at the [`make_transfer()`](https://crates.parity.io/srml_balances/struct.Module.html#method.make_transfer) function, it both "verifies" and "writes", which means you will need to be careful exactly where you include it as a part of your module's logic.

```rust
// end of verifications

<balances::Module<T>>::make_transfer(&from, &to, value)?;

// beginning of writing to storage
```

Note that in our template, we chain execute the `transfer_from()` function after `make_transfer()`, and `transfer_from()` also has the ability to fail...

```rust
// nothing after this line should fail
<balances::Module<T>>::make_transfer(&from, &to, value)?;
// but this function technically "could" fail
Self::transfer_from(owner.clone(), sender.clone(), kitty_id)?;
```

So are we breaking our own rules here?

Well... not exactly. If you can make checks ahead of time to guarantee that this function will not fail, then we have not broken any rules of runtime development.

Let's take a second look at the checks which could fail within `transfer_from()`:

* No owner exists for the kitty
* The "from" account does not own the kitty in question
* There will be an underflow when subtracting the kitty from the user's `owned_kitty_count`
* There will be an overflow when adding the kitty to a user's `owned_kitty_count`

So, before we can chain the functions like we want, we need to make sure that all of these checks will succeed. This is actually pretty easy!

1. Make sure your `buy_kitty` function checks that the kitty has an owner
2. Use that owner value to directly power your `transfer_from()` function
3. Be satisfied that if there exists an owner for a kitty, then that owner's kitty count is greater than 0. (or else you have a bigger problem in your runtime than this)
4. Be satisfied that the `all_kitties_count` and the `owned_kitties_count` uses the same type (`u64`) to track the number of kitties. Our mint function, which is the only way that new kitties come into existence, ensures that `all_kitties_count` will never overflow. Thus, we can say confidently that the `owned_kitties_count` will never overflow since this number must be less than or equal to the `all_kitties_count`.

So, in the context of our `buy_kitty()` function, we can actually use `expect()` with a proof of why this function cannot fail. Like so:

``` rust
Self::transfer_from(owner.clone(), sender.clone(), kitty_id)
    .expect("`owner` is shown to own the kitty; \
    `owner` must have greater than 0 kitties, so transfer cannot cause underflow; \
    `all_kitty_count` shares the same type as `owned_kitty_count`, \
    which means transfer cannot cause an overflow; \
    qed");
```

You will actually find proofs just like this [scattered throughout the substrate repository](https://github.com/paritytech/substrate/search?q=expect).

Remember, as a blockchain developer, it is your duty to verify the sanctity of your code and logic within it. Substrate is not a framework built to protect you from errors, like a smart contract platform may provide.

Take a second to lean back in your chair, and ponder this last section, as it will be important to keep in mind when you start to develop your own projects with substrate.

## Your Turn!

Follow the template provided to program in the necessary code to complete the `buy_kitty()` function. Feel free to test your new function in the Polkadot-JS Apps UI for any bugs.

<!-- tabs:start -->

#### ** Template **

[embedded-code](./assets/3.3-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](./assets/3.3-finished-code.rs ':include :type=code embed-final')

<!-- tabs:end -->
