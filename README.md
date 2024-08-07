# Buy Kitty Extrinsic

Now that kitties can have a price, we want to enable them to be purchaseable.

For that, we will create the `buy_kitty` extrinsic.

## Max Price

You will notice in our scaffolding, one of the parameters of our `buy_kitty` extrinsic is `max_price: BalanceOf<T>`.

This allows the buyer to set the maximum price they are willing to spend to buy a specific kitty.

But this is a little strange right? The kitty already has a price set on it. Why would we need the buyer to also send us a price?

Well, without this `max_price` parameter, you open users of your pallet to a nasty attack!

Remember that transactions on a blockchain are bundled into a block and executed one by one.

Also remember that the order of transactions in the block is up for interpretation by the block producer!

Imagine there is a kitty, and its price is set to 10 DOT. Then someone goes and submits `buy_kitty` without there being a `max_price` parameter.

What can happen is that the kitty owner could see that transaction in the transaction pool, before the transaction is included in the block, then submit their own high priority transaction (usually by increasing the fees they pay), which raises the price of the kitty.

Do you see the problem now?

Basically without the `max_price` parameter, your pallet will become a honeypot for attackers to trick users into sending them their whole balance.

The `max_price` parameter ensures the buyer and seller have reached an agreement on price, and that any changes to the agreement would not be executed.

## Your Turn

Hopefully you can see some of the ways that programming for a blockchain is different than what you might expect. Remember that blockchain systems need to be resilient to malicious individuals. Because it is a public and open system, we cannot make any assumptions about the behaviors of our users. We can only influence those behaviors through the rules of our state transition function.

In this step, we again will just scaffold what we need for the `buy_kitty` extrinsic.

- Create a new event `Sold` with the fields noted in the template.
- Create a new extrinsic `buy_kitty` with the params noted in the template.
- Create a new internal function `do_buy_kitty` which simply deposits the `Sold` event.
