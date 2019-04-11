Playing Our Game
===

If you have made it this far, **congratulations**!

You have now completed the development of your Substratekitties runtime module. If you followed our instructions carefully and made all of the right checks, your code should look something like what we have shown here.

This would be a good time to check your work using the Polkadot-JS Apps UI, ensuring you have not run into any errors or introduced any problems.

## Manual Tests

You should run the following manual tests:

- Fund multiple users with tokens so they can all participate
- Have each user create multiple kitties
- Try to transfer a kitty from one user to another using the right and wrong owner
- Try to set the price of a kitty using the right and wrong owner
- Buy a kitty using an owner and another user
- Use too little funds to purchase a kitty
- Overspend on the cost of the kitty and ensure that the balance is reduced appropriately
- Breed a kitty and check that the new DNA is a mix of the old and new
- After all of these actions, confirm that all users have the right number of kitties, the total kitty count is correct, and any other storage variables are correctly represented

## Challenge

Our challenge to the reader is to extend the Substratekitties runtime and include additional functions and features you might want to see in this runtime module.

Here are some ideas:

- Track the parents of a kitty during when `breed_kitty()` is called. Maybe as just an event...?

- Limit the number of kitties that can be created by `create_kitty()`, and/or add a price curve to make each new kitty cost more to create.

- Add a cost to breeding kitties that the person receiving the new kitty has to pay. Make sure funds are correctly sent to each user. Make sure a person can breed using their own kitties.

- Add a kitty "fight" where two kitties can compete to win a game based on a random number and some weighted statistic. The winning kitty has their stats increase, giving them a better chance to win the next round.

- Add a gene mutation algorithm to kitty breeding which will introduce traits that neither of the parent kitties had.

- Introduce an auction system for owners who do not want to simply set a fix price to purchase their kitty. Auctions close after a period of time has passed where no one has placed a bid.

What other ideas can you think of?

In the next section we will be creating a custom UI for this game using Substrate JavaScript libraries. This custom UI is sensitive to changes in naming conventions, so you may need to copy the final Substratekitties code to ensure compatibility.

<!-- tabs:start -->

#### ** Solution **

[embedded-code-final](../../3/assets/3.5-finished-code.rs ':include :type=code embed-final')

<!-- tabs:end -->