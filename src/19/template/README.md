# Generate Unique DNA

In this step, we will show how to generate uniqueness using information from the blockchain.

## Randomness

Ideally, we would give every kitty we mint a randomly generated DNA.

However generating randomness on a blockchain is extremely difficult because we must come to consensus over all logic and data used on the blockchain. Any kind of randomness function must generate exactly the same randomness for all nodes. And if that is the case, it is still possible to influence randomness as a block producer by choosing NOT to build a block with a randomness you do not like.

Polkadot does provide access to a verifiable random function (VRF), but exactly the properties of this VRF and how to use it is beyond the scope of this tutorial. Not to mention we are also iteratively improving the VRF provided by Polkadot.

## Uniqueness

So rather than using true randomness, we will instead try to generate uniqueness.

There are different levels of uniqueness we can achieve using data from our blockchain.

- `frame_system::Pallet::<T>::parent_hash()`: The hash of the previous block. This will ensure uniqueness for every fork of the blockchain.
- `frame_system::Pallet::<T>::block_number()`: The number of the current block. This will obviously be unique for each block.
- `frame_system::Pallet::<T>::extrinsic_index()`: The number of the extrinsic in the block that is being executed. This will be unique for each extrinsic in a block.
- `CountForKitties::<T>::get()`: The number of kitties in our blockchain.

If we combine all of these things together, we can ensure that every kitty we mint will be unique, no matter:

- Which block it comes in.
- How many extrinsics are in a block.
- Or even if a single extrinsic mints multiple kitties.

## Hash

Obviously our uniqueness inputs are not super useful as is. But we can convert these inputs into a unique set of bytes with fixed length using a Hash function.

```rust
// Collect our unique inputs into a single object.
let unique_payload = (item1, item2, item3);
// Encode that object into a vector of bytes.
let encoded_payload: Vec<u8> = unique_payload.encode();
// Hash those bytes to create a fixed-size hash.
let hash: [u8; 32] = frame_support::Hashable::blake2_256(&encoded_payload)
```

Another nice thing about using a hash is you get some sense of pseudo-randomness between the input and output. This means that two kitties which are minted right after one another could have totally different DNA, which could be useful if you want to associate unique attributes to the different parts of their DNA. 🤔


## Your Turn

Now that you know how to acquire uniqueness from your blockchain, and how to hash those items, create a new function called `fn gen_dna() -> [u8; 32];` which does these steps to create unique DNA for each kitty that is minted.

Update your `create_kitty` extrinsic to generate and use this unique DNA.
