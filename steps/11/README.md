# Blockchain Storage

Blockchains use a Merkle Trie structure to store data. The Merkle Trie provides two important properties for blockchains:

1. Allows the whole database to be represented by a single fingerprint, which can easily be compared to other nodes.
2. Allows the creation of lightweight proofs, proving that specific data exists in the database.

This comes at the cost of additional complexity reading and writing data to the blockchain.

Let's learn about Merkle Tries in more detail.

## Hash Functions

Hash functions are an important tool throughout blockchain development.

A hash function takes an arbitrary sized input and returns a fixed-size string of bytes.

This output, usually called a hash, is unique to each unique input. Even a small change to the input creates a dramatic change to the output.

Hash functions have several key properties:

- Deterministic: The same input always produces the same output.
- Pre-image Resistant: It is difficult to derive the original input from its hash value.
- Collision Resistant: It’s hard to find two different inputs that produce the same hash output.

These properties make hash functions key for ensuring data integrity and uniqueness in blockchain technology.

## Hash Fingerprint

Due to the properties of a Hash, it is often referred to as a fingerprint.

For context, a 32-byte hash has 2^32 different possible outputs. This is nearly as many atoms as there are in the whole universe!

This uniqueness property helps blockchain nodes come to consensus with one another.

Rather than needing to compare all the data in their blockchain database with one another, they can simply share the hash of that database, and know in a single small comparison if all data in that database is the same.

Remember, if there were any small differences between their databases, even just one bit in a multi-terabyte database being different, the resulting hash would dramatically change, and they would know their databases are not the same.

## Merkle Trie

A merkle trie is a data structure which is constructed using a hash function.

Rather than hashing the whole database into a single hash, we create a tree of hashes.

For example, we take pairs of data, combine them, then hash them to generate a new output. Then we take pairs of hashes, combine them, then hash them to generate another new output.

We can repeat this process until we are left with a single hash called the "root hash". This process literally creates a tree of hashes.

Just like before, we can use a single hash to represent the integrity of all data underneath it, but now we can efficiently represent specific pieces of data in the database using the path down the trie to that data.

It is called a merkle "trie" because the trie data structure is used to reduce the amount of redundant data stored in the tree.

### Complexity

The reason we go into this much detail about merkle tries is that they increase the complexity in reading and writing to the blockchain database.

Whereas reading and writing to a database could be considered `O(1)`, a merklized database has read and write complexity of `O(log N)`, where `N` is the total number of items stored in the database.

This additional complexity means that designing storage for a blockchain is an extremely important and sensitive operation.

The primary advantage of using a merkle trie is that proving specific data exists inside the database is much more efficient! Whereas you would normally need to share the whole database to prove that some data exists, with a merklized database, you only need to share `O(log N)` amount of data. This is very important to support light clients.

In this next section, and throughout the tutorial, we will start to explore some of those decisions.
