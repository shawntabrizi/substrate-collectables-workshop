# Track Owned Kitties

Now that we can generate unique kitties, we need to consider all the ways we need to store and track those kitties.

## Redundant Storage

As a rule, you only want to store data in your blockchain which is necessary for consensus. Blockchains are extremely slow, low powered, and expensive. Blockchains are extremely good at one thing: achieving agreement among a decentralized and untrusted set of individuals.

When building a blockchain, you need to think about the various constraints

TODO

## Your Turn

Now that you understand the tradeoffs associated with creating redundant storage, let's make a new `StorageMap` called `KittiesOwned` which can help us more easily find what kitties an account is the owner of.

- TODO:
	- Explain advantages and disadvantages of redundant storage
	- Explain tradeoffs with UIs
	- Explain iteration over maps and vecs
