Introduction to Kitty Auctions
===

Auctions are compelling use cases for blockchains in general and Substrate Runtime provides a good platform to implement this. Let's introduce an auction system for kitty owners who do not want to simply set a fix price for their kitty.

We will implement an [English auction](https://en.wikipedia.org/wiki/Auction#Types), where new bids increase the price until the auction ends at a fixed time, and the account who bid the highest price wins the auction.

First, the owner of a kitty will create an auction with 3 parameters:
1. The kitty id
2. The minimum bid (the base price for the kitty)
3. An expiration time (as a block number)

Second, the accounts who want this kitty will bid with 2 parameters:
1. The kitty id
2. The price they want to bid for that kitty

Finally, the account with the highest bid will win when the auction ends, that is when our chain's current block number reaches the auction expiration time in blocks. We will finalize the auction by paying from bidder to the kitty owner, and transferring the kitty to the highest bidder. We will clean up all the bids and the auction from the storage.

In this section, you will learn the following technical concepts:
- What is the concept of time on chain?
- How do I create an escrow system using reserve balance?
- How do I implement a process that needs to be done at the start or end of a block?

---

Thank you [@arikan](https://github.com/arikan) for creating this content!
