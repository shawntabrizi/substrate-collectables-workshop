# Track Owned Kitties

Now that we can generate unique kitties, we need to consider all the ways we need to store and track those kitties.

## Redundant Storage

As a rule, you only want to store data in your blockchain which is necessary for consensus. Blockchains are extremely slow, low powered, and expensive. Blockchains are extremely good at one thing: achieving agreement among a decentralized and untrusted set of individuals.

When building a blockchain, you need to think about the various constraints:

- Execution Constraints
- Memory Constraints
- Storage Constraints
- Bandwidth Constrains
- etc...

In the case of adding redundant storage, all of these constraints come into play! So let's talk about that a bit.

### Iteration

It is common when writing code that you might want to perform iteration over items stored in your blockchain.

In general iteration should be avoided where possible, but if unavoidable it is critical that iteration be bounded in size.

We literally cannot allow code on our blockchain which would do unbounded iteration, else that would stall our blockchain, which needs to produce a new block on a regular time interval.

#### Maps

When you store and iterate over a map, you need to make two considerations:

1. That the map may not have a bounded upper limit.
2. That each access to the map is very expensive to the blockchain (each is a unique read to the merkle trie).

If you want to do iteration, probably you do NOT want to use a map for exactly these reasons.

Maps are great instead for when you need to access or manipulate a single item at a time.

#### Vec

When you store and iterate over a vector, the only real consideration you need to have is how large that vector is.

Accessing large files from the database is going to be slower than accessing small files.

Once you access the vector, iterating over it and manipulating it is relatively cheap compared to any kind of storage map (but not zero, complexity about vector access still applies).

If you want to do iteration, you definitely would prefer to use a vector.

#### Middle Ground

But sometimes you need to iterate over data, and store a lot of data. This is where we can do a middle ground.

While it is not great for your Storage Constraints to store redundant data, it can be much better for all your other constraints.

Let's say we want to answer the question: Give me all the kitties owned by Shawn.

If all the kitties are stored just in the map, then we would need to iterate over all of them to find which ones are owned by Shawn.

However, if we ALSO store a vector of kitties owned by Shawn in another storage, yes we would have redundant information in our database, but we will also be able to answer this question much more efficiently.

A key part of designing your storage is making it efficient for the tasks your code will need to execute. Similarly, you will need to design your code to be efficient for the storage constraints you have.

Honestly, its a lose / lose situation most times, but it is a part of what we need to do when designing blockchain systems.

## Storage Optimizations

Storing vectors is a pretty normal part of Pallet development, and there are ways we can optimize adding items to a vector.

Let's look at a naive way to add a new item to the vector:

```rust
let mut owned_kitties: Vec<[u8; 32]> = KittiesOwned::<T>::get(owner);
owned_kitties.append(new_kitty);
KittiesOwned::<T>::insert(owner, owned_kitties);
```

The first call we need to make is `get` which returns to us all the data in the vector, and all that data is stored in a merkle trie in a database that is really expensive to read from.

Then we add the item to the vector, and then write the whole vector back into storage.

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
