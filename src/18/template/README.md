# Kitty Struct

In this step, we will create a new struct which is generic over `<T: Config>`. This step will just be using basic Rust, so no macro-magic or Pallet specific stuff is happening here.

## Creating a Struct

Creating a new struct in Rust is pretty straight forward.

```rust
pub struct Kitty {
	dna: [u8; 32],
	owner: u32,
}
```

One important detail for creating a struct to be used in a Pallet is that the struct should be marked `pub`. The compiler will complain if you try to use an object in your pallet without marking it `pub`, because it will be used across other modules in your blockchain.

Creating a new instance of this struct is easy:

```rust
let kitty = Kitty {
	dna: [0u8; 32],
	owner: 0u32,
}
```

## Creating a Generic Struct

We will often want to access our Pallet specific types and store them. To do this, we need to make a struct which is generic over those types.

The example we will use here is wanting to store the account of the owner of the kitty.

There are actually two ways we can do this, and we will show you both.

### Generic Over Each Type

The first, and most verbose option in our situation is to make the struct generic over each type we want to use.

```rust
pub struct Kitty<AccountId> {
	dna: [u8; 32],
	owner: AccountId,
}
```

If we want to use multiple generic types, we could just keep extending this pattern:

```rust
pub struct Kitty<AccountId, BlockNumber> {
	dna: [u8; 32],
	owner: AccountId,
	created: BlockNumber,
}
```

When you reference this type, you will need to mention the generics that it is using:

```rust
pub type MyAlias<T: Config> = Kitty<T::AccountId, T::BlockNumber>;
```

The problem with this approach is that as you use more and more types, everything will just become excessively verbose.

### Generic Over `T`

The more common option is to make a struct generic over `<T: Config>` instead of the individual types. Through `T` you will then be able to access ALL the types from our blockchain. This is exactly why we created `T` to begin with!

Let's look how that might look like:

```rust
pub struct Kitty<T: Config> {
	dna: [u8; 32],
	owner: T::AccountId,
}
```

In this context, you can see you can access any of the runtime types by doing `T::Type`.

It also becomes a lot easier to reference this type, no matter how many different runtime types you are using:

```rust
pub type MyAlias<T: Config> = Kitty<T>;
```

I want to be clear, in the final compiled binary, both options for creating a generic struct are exactly the same. There are some nuanced advantages to both options, but these details are really at a much lower level than we will go over in this tutorial.

## Your Turn

Now that you know how to create a generic struct, create a new `Kitty` struct which is generic over `<T: Config>`. It should have fields for the `dna` of the kitty and the `owner` of the kitty.

In our next step, we will learn how we can actually use this struct in runtime storage.
