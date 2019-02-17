Auction a Kitty
===

Let's introduce an auction system for kitty owners who do not want to simply set a fix price for their kitty.

We will implement an [English auction](https://en.wikipedia.org/wiki/Auction#Types), where new bids increase the price until the auction ends at a fixed time, and the account who bid the highest price wins the auction.

First, the owner of a kitty will create an auction with 3 parameters: the kitty id, minimum bid (the base price for the kitty), and an expiration time.

Second, the accounts who want this kitty will bid with 2 parameters: the kitty_id and the price they want to bid for that kitty.

Finally, we will check if the auction expiration time is reached and finalize the auction by transferring the kitty to the highest bidder and paying from bidder to the kitty owner. The auction will close after the expiration time has passed even when no one has placed a bid.

## Create an Auction

We learned how to create a custom struct for our runtime earlier. This time we will create another one for our Auction. In addition to other types we used earlier, the Auction struct will use a new substrate specific type `BlockNumber`, which will help us trigger finalizing the auction when its expiration time is reaches a certain block number. For example, you can get the current block height from the SRML system module:

```rust
let current_block_number = <system::Module<T>>::block_number();
```

An `Auction` struct should have the following properties:
- `kitty_id`: `Hash`
- `kitty_owner`: `AccountID`
- `expiry`: `BlockNumber`
- `min_bid`: `Balance`
- `high_bid`: `Balance`
- `high_bidder`: `AccountId`

After adding the Auction struct, create a storage item for the auction so that it can be retrieved by a `kitty_id`. Notice there may be many kittes each with an open auction that expires at the same block height. So you should store a list of auctions which have the same `expiry` value as a block number. This will also help optimize the finalization of auctions later.

Finally, we don't want infinitely long auctions to overflow our storage, so we should add a variable to limit long auction expiration periods. You can set this variable as a `BlockNumber`. Assuming a block is produced every 5 seconds and you want 24 hours as your limit, you can set it as 17280 = 24 * 60 * 12.

```rust
let a_future_block_number = T::BlockNumber::sa(17280);
```

Now that we created our storage items for auction, we can implement the `create_auction()` method in our module. Aside from the sender signature and kitty ownership checks, we want to check if the new auction's duration is within the limit we configured. The owner of the kitty will set the auction expiry in minutes, so you need to convert this input into block number, like the way we did above.

Then, we will create an Auction object and set its values. We will set initial values for `high_bid` same as `min_bid`, and `high_bidder` same as `kitty_owner`, so while resolving an auction, we will guarantee that high_bidder != kitty_owner.

Finally, we will add this new auction to our storage. Follow the template for implementing these steps.

## Auction bidding

Now, we want people to bid prices for our kitty auction. We will add a `bid_auction()` method for handling bids. We want to ensure this cat has an existing auction, make sure the bid is greater than minimum required bid as well as the existing high bid, so that we can store it as the new high bid.

If this incoming bid is higher than the existing high bid, we can set its respective values as the new `high_bid` and `high_bidder`. To complete this function, we should insert these new values into our storage mappings. Follow the template for implementing these steps.

## Finalize auctions

The `decl_module!` can take a special function called `on_finalise()`, which is used for anything that needs to be done at the end of the block ([learn more](https://substrate.readme.io/docs/decl_module#section-on_initialise-and-on_finalise-)). We can use this for finalizing an auction whose expiration time is equal to the current block number.

Remember, we may have multiple auctions that will expire at a given block number, so from our storage we will retrieve all auctions that will expire at this current block we are in.

```rust
let auctions = Self::auctions_expire_at(<system::Module<T>>::block_number());
```

We will go through each auction and resolve it: first, we transfer the kitty from its `owner` to the winning `high_bidder`, then we pay the final bid price from the bidder to the owner.

Finally, we should cleanup the storage by removing the resolved auction from `<KittyAuction>` and `<Auctions>`.

## Your Turn!

Follow the template provided to program in the necessary code to complete the Auction functionality.

Start from the Auction struct and your runtime storage to store the auctions.

The `create_auction()`, `bid_auction()`, and `on_finalise()` skeleton functions are ready for you, but you will need to add the logic.

## Manual Tests

Once you complete your auction functionality, you should run the following manual tests using the Polkadot-JS Apps UI. Make sure you add the Auction structure to the previous JSON import, so that the page will be able to decode the information correctly.

- Create an auction for a kitty.

- Bid for this auction with other accounts.

- Check if the auction is resolved when its expiration block number is reached.

> REMINDER: Remember to reset your chain so that you start of fresh when interacting with the UI.

<!-- tabs:start -->

#### ** Template **

[embedded-code](./assets/3.6-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](./assets/3.6-finished-code.rs ':include :type=code embed-final')

<!-- tabs:end -->

