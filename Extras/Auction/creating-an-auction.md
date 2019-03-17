Create an Auction
===

We learned how to create a `Kitty` struct for our runtime [back in chapter 1](/1/storing-a-structure.md). This time, we will create another struct for an auction. In addition to other Substrate specific types we used earlier, the `Auction` struct will use the `BlockNumber` type, which will help us trigger the end of an auction.

An `Auction` struct should have the following properties:
- `kitty_id`: `Hash`
- `kitty_owner`: `AccountID`
- `expiry`: `BlockNumber`
- `min_bid`: `Balance`
- `high_bid`: `Balance`
- `high_bidder`: `AccountId`

After adding the `Auction` struct, create a storage item for the auction so that it can be retrieved by a `kitty_id`.

Notice there may be many kitties each with an open auction that expires at the same block height. So you should store a list of auctions that is mapped to an `expiry` value. This will also help optimize the finalization of auctions later. The `expiry` should be in `BlockNumber` type rather than time in minutes or hours, so that our auction system can be used in other chains with different block times.

## Block Number

In your implementation you can read the current block number (aka block height) and set a new value in `BlockNumber` type.

We don't want infinitely long auctions to overflow our storage, so we should add a variable `AuctionPeriodLimit` to limit long auction expiration periods. You can set this limit variable as a `BlockNumber`. For example, assuming a block is produced every 5 seconds in your chain and you want 24 hours as your limit, you can set it as (24 hours/day * 60 minutes/hour * 60 seconds/min) / 5 seconds/block = 17280 blocks per day.

```rust
let a_future_block_number = T::BlockNumber::sa(17280);
```

Later, we will compare this limit against the current block height from the SRML system module:

```rust
let current_block_number = <system::Module<T>>::block_number();
```

## Auction creation

Now that we created our storage items for auction, we can implement the `create_auction()` method in our module. Aside from the sender signature and kitty ownership checks, we want to check if the new auction's duration is within the limit we configured.

Then, we will create an `Auction` object and set its values. Note that we should set initial values for `high_bid` same as `min_bid`, and `high_bidder` same as `kitty_owner`. So, if these default values are not updated, that means no one bid for this auction. So, while resolving an auction, we will check that `high_bidder != kitty_owner`.

Then, we will add this new auction object into our runtime storage `KittyAuction` and `Auctions`.

Finally, we want to deposit the `AuctionCreated` event stating to the world that an auction is created.

<!-- tabs:start -->

#### ** Template **

[embedded-code](./assets/5.1-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](./assets/5.1-finished-code.rs ':include :type=code embed-final')

<!-- tabs:end -->
