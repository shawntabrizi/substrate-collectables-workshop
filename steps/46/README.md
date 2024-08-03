# Buy Kitty Logic

Now that we have scaffolded the `buy_kitty` extrinsic, its time for us to program it's logic.

## Sanity Checks

As always, before we execute any logic, we should check that everything is okay to actually proceed with the kitty purchase.

The first check should obviously be if the kitty the user wants to purchase actually exists.

Then we should check if the kitty is actually for sale, which is indicated by `Some(real_price)`, and extract that `real_price`.

Finally, we should check that there is an agreement between the buyer and the seller on the price.

The buyer will submit `max_price`, and we want to ensure that this `max_price` is greater than or equal to the `real_price`.

## Transfer Logic

To execute a purchase, we need to transfer two things:

- The token balance from the buyer to the seller.
- The kitty from the seller to the buyer.

### Transfer the Native Balance

To transfer the `NativeBalance`, you can use the [`transfer`](https://paritytech.github.io/polkadot-sdk/master/frame_support/traits/tokens/fungible/trait.Mutate.html#method.transfer) API which is included in the `fungible::Mutate` trait.

```rust
fn transfer(
    source: &AccountId,
    dest: &AccountId,
    amount: Self::Balance,
    preservation: Preservation
) -> Result<Self::Balance, DispatchError>
```

> NOTE: To access this function, you will need import the trait to bring it in scope. Otherwise you will get an error that the function does not exist. So don't forget to include:
>
> ```rust
> use frame_support::traits::fungible::Mutate;
> ```

The first 3 parameters here are easy enough to understand: `source`, `dest`, and `amount`.

However we also have a 4th parameter which is `preservation: Preservation`.

```rust
/// The mode by which we describe whether an operation should keep an account alive.
pub enum Preservation {
	/// We don't care if the account gets killed by this operation.
	Expendable,
	/// The account may not be killed, but we don't care if the balance gets dusted.
	Protect,
	/// The account may not be killed and our provider reference must remain (in the context of
	/// tokens, this means that the account may not be dusted).
	Preserve,
}
```

To understand this, you will need to dive deep into dust accounts, existential deposit, and account deletion.

That is all beyond the scope of this tutorial, but the high level idea is that we require accounts in the `polkadot-sdk` to maintain a minimum balance. Whenever we transfer funds from the user, beyond checking they have enough to transfer, we also check whether that minimum amount is still left over in their account, and adjust the result of our transfer based on our `Preservation` requirements.

In this context, we don't want someone to kill their account to buy a kitty, so we want to use `Preservation::Preserve` for our `transfer`.

So the final syntax should look like:

```rust
T::NativeBalance::transfer(&buyer, &kitty.owner, real_price, Preservation::Preserve)?;
```

### Transfer the Kitty

To transfer the kitty, we can simply reuse our `Self::do_transfer` function that we wrote in the past.

Hopefully you can start to see the payoff of how we have organized our code.

As you work on more complex projects, breaking down behaviors into simple to understand and reusable pieces of internal logic will not only make your code more clean, but also easier to audit and update.

It is unlikely you will have foresight on exactly how to break down your project when you first start writing it, but it is definitely something you should keep in mind as you find yourself writing potentially duplicate code.

### Propagate Up Errors

Both transfer functions need to succeed for the sale to complete successfully.

If either one of them would fail, the whole purchase should fail.

Thankfully, both of our transfer functions return a result, and to handle things correctly here, we just ned to propagate up those errors. For that, we simply include `?` at the end of the function.

If at any point our extrinsic or the logic inside the extrinsic returns an error, the whole extrinsic will fail and all changes to storage will be undone. This is exactly the same behavior you would expect from a smart contract, and keeps our state transition function functioning smoothly.

## Real Price

As a final change in your pallet logic, we should update the `Event::<T>::Sold` to emit the `real_price` instead of the buyers `max_price`.

This is perhaps the first really good use case for emitting an Event.

In all of our previous events, we basically just deposit the data that we already get from the extrinsic.

However in this case, our `buy_kitty` extrinsic actually uses internal logic to manipulate what actually happens. It could be that the user submits a `max_price` higher than the `real_price`, and thus the extrinsic and a success message would not actually tell us what happened exactly.

In this case, we can actually report by to the user and any UI they are using the executed sale price of the kitty.

So this is really where Pallet Events shine, and how they should be used. This is something you will develop a sense for over time.

## Your Turn

You now have all the tools and information needed to build your `do_buy_kitty` logic.

- Add the sanity checks needed to make sure `do_buy_kitty` should execute at all.
- Transfer both the `NativeBalance` and the `Kitty`, being sure to check for success.
- Finally, update the `Event::<T>::Sold` to deposit the `real_price` that was transferred.
