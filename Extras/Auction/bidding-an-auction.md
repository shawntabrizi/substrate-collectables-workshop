Bidding an auction
===

Now that we have a method for creating auctions for kitties, we want users to bid prices for our auctions. We will escrow the user's bid, and track how much we took from them so that we guarantee the payment when the user wins. This also means we have to return all funds to losing users at the end of the auction.

First, we will add two new storage items for bids:

* `Bids` for tracking which account bid how much for which kitty auction
* `BidAccounts` for storing all the accounts that bid for each kitty auction

Then, we will implement a `bid_auction()` method for handling bids. We want to ensure that:

* the cat has an existing auction and it is not expired
* the bid is greater than the existing high bid
* the bidder has enough free balance for this bid

After these verifications, we know that the new incoming bid is the highest one, so we will update the auction object's `high_bid` and `high_bidder` values with the new values. Then, we will update the storage items `KittyAuction` and `Auctions` with this updated auction object.

## Using reserve balance for escrow

We will add the sender's bid to the `Bids` list and escrow this amount from their account, so that the payment is guaranteed if the user wins the auction. For this purpose we can use the balances module's `reserve` function, which sets aside a balance amount still 'owned' by the account holder that cannot be transferred, yet can be used by subsystems, like our auction logic.

```rust
<balances::Module<T>>::reserve(sender, amount)?;
```

In the next step, while finalizing the function, the reserved balaced will be released using the balances module's `unreserve` function.

```rust
<balances::Module<T>>::unreserve(sender, amount)?;
```

We should make sure that new bids from the same user take into account how much we have already escrowed.

Follow the template for implementing these steps.


<!-- tabs:start -->

#### ** Template **

[embedded-code](./assets/5.2-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](./assets/5.2-finished-code.rs ':include :type=code embed-final')

<!-- tabs:end -->
