# Storage Values

The most basic storage type for a blockchain is a single `StorageValue`.

A `StorageValue` is used to place a single object into the blockchain storage.

A single object can be as simple as a single type like a `u64`, or more complex structures, or even vectors.

What is most important to understand is that a `StorageValue` places a single entry into the merkle trie. So when you read data, you read all of it. When you write data, you write all of it. This is in contrast to a `StorageMap`, which you will learn about next.

## Construction

We constructed a simple `StorageValue` for you in the code, but let's break it down:

```rust
#[pallet::storage]
pub(super) type CountForKitties<T: Config> = StorageValue<Value = u64>;
```

As you can see, our storage is a type alias for a new instance of `StorageValue`.

Our storage value has a parameter `Value` where we can define the type we want to place in storage. In this case, it is a simple `u64`.

You will also notice `CountForKitties` is generic over `<T: Config>`. All of our storage must be generic over `<T: Config>` even if we are not using it directly. Macros use this generic parameter to fill in behind the scene details to make the `StorageValue` work. Think about all the code behind the scenes which actually sticks this storage into a merkle trie in the database of our blockchain.

Visibility of the type is up to you and your needs, but you need to remember that blockchains are public databases. So `pub` in this case is only about Rust, and allowing other modules to access this storage and its APIs directly.

You cannot make storage on a blockchain "private", and even if you make this storage without `pub`, there are low level ways to manipulate the storage in the database.
