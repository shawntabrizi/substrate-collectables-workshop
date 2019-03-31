Finalizing an auction
===

The `decl_module!` can take a special function called `on_finalise()`, which is used for anything that needs to be done at the end of the block ([learn more](https://substrate.readme.io/docs/decl_module#section-on_initialise-and-on_finalise-)). We can use this for finalizing an auction whose expiration time is equal to the current block number.

Remember, we may have multiple auctions that will expire at a given block number, so from our storage we will retrieve all auctions that will expire at this current block we are in.

```rust
let auctions = Self::auctions_expire_at(<system::Module<T>>::block_number());
```

In the `on_finalise` function, we will go through each auction and resolve it:

1. Before writing to storage we check kitty owner, high bidder, and make sure the bidder is not the kitty owner (if they are equal, it means no bidders).

2. The winning bidder's reserved balance will be freed using the `unreserve` function.

3. We pay the final bid price from the bidder to the kitty owner.

4. We transfer the kitty from its `owner` to the winning `high_bidder`.

5. We cleanup the storage by removing the resolved auction and its bids from `<Bids>`, `<BidAccounts>`, `<KittyAuction>`, and `<Auctions>`.

6. We free the escrow amounts of all losing bidders using the `unreserve` function.

We also add a check to `transfer_from` function for disabling any sales of the kitty while there is an open auction.

## Bounding loops while finalizing a block

If you let the users of your chain to create arbitrary amount of auctions that will end in a certain block, than all those auctions will be looped in the `on_finalise()` function. This is an attack vector, it can halt your chain. So, you should prevent this possibility by introducing a limit to the number of auctions that will end in a block. So you should do the following:

1. Add a constant `MAX_AUCTIONS_PER_BLOCK` to limit the number of auctions per block.

2. Ensure the number of auctions that will expire in the current block does not exceed the limit of max auctions per block.

## Your Turn!

The `on_finalise()` skeleton functions are ready for you to add logic. Follow the template provided to program in the necessary code to complete the kitty auction functionality.

As a further execrice, you can implement a minimum bid increment for the auctions. Without a minimum bid increment, the auction might last forever as somebody can drive up the price by ultra small units in each block.

## Manual Tests

Once you complete your auction functionality, you should run the following manual tests using the Polkadot-JS Apps UI.

- Create an auction for a kitty.

- Create bids for this auction from other accounts.

- Check if bidder's bid amount is reserved and the bid is recorded in Bids. Also check if the reserved balance is increased correctly as the same bidder increases the bid (the reserve balance should be equal to the latest bid amount of the account).

- Check if the auctioned kitty is sold to the high bidder for the high bid when the auction is finalized (high bidder should be the new owner, and the previous owner and bidder balances should be updated).

- Check if the losing bidders' reserved balances are freed.

> REMINDER: Make sure you add the Auction structure to the previous JSON import, so that the Polkadot-JS Apps UI will be able to decode the information correctly.

> REMINDER: Remember to reset your chain so that you start of fresh when interacting with the UI.

<!-- tabs:start -->

#### ** Template **

[embedded-code](./assets/5.3-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](./assets/5.3-finished-code.rs ':include :type=code embed-final')

<!-- tabs:end -->
