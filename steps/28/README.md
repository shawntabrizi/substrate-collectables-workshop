# Track Owned Kitties

Now that we can generate unique kitties, we need to consider all the ways we need to store and track those kitties.

## Redundant Storage

As a rule, you only want to store data in your blockchain which is necessary for consensus. Blockchains are extremely slow, low powered, and expensive. Blockchains are extremely good at one thing: achieving agreement among a decentralized and untrusted set of individuals.

When building a blockchain, you need to think about the various constraints

TODO

## Iteration

TODO

### Maps

TODO

### Vec

TODO

## Storage Optimizations

Storing vectors is a pretty normal part of Pallet development, and there are ways we can optimize adding items to a vector.

Let's look at a naive way to add a new item to the vector:

```rust
let mut owned_kitties: Vec<[u8; 16]> = KittiesOwned::<T>::get(owner);
owned_kitties.append(new_kitty);
KittiesOwned::<T>::insert(owner, owned_kitties);
```

The first call we need to make is `get` which returns to us all the data in the vector, and all that data is stored in a merkle trie in a database that is really expensive to read from.

Then we add the item to the vector, and then write the whole new item back into storage.

But this is way more inefficient than we need! We don't actually need to know what is inside the vector to add a new item to it, we can just say "add this item".

So we can convert the whole logic to:

```rust
KittiesOwned::<T>::append(owner, new_kitty);
```

In this case, we use our own storage abstractions to avoid needing to read the whole vector in our runtime logic to simply add a new item to it.

Fun fact, this optimization actually lead to a 95% performance increase for the `polkadot-sdk` back before Polkadot launched and we were benchmarking it!

## Your Turn

Now that you understand the tradeoffs associated with creating redundant storage, let's make a new `StorageMap` called `KittiesOwned` which can help us more easily find what kitties an account is the owner of.

Then let's update the `mint` function to `append` the kitty's DNA to the `KittiesOwned` vector for the `owner`.

- TODO:
	- Explain advantages and disadvantages of redundant storage
	- Explain tradeoffs with UIs
	- Explain iteration over maps and vecs
