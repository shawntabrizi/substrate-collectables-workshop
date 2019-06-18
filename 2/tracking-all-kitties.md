Tracking All Kitties
===

Now that we have enabled each user to create their own unique kitty, we should probably start tracking them!

It makes sense for our game to track a total number of kitties created, as well as a way to track who owns each kitty.

## "Verify First, Write Last"

> IMPORTANT: This section is important

As a developer building on Substrate, it is critical that you make a distinction about how you should design your runtime logic versus developing a smart contract on a platform like Ethereum.

On Ethereum, if at any point your transaction fails (error, out of gas, etc...), the state of your smart contract will be unaffected. However, on Substrate this is not the case. As soon as a transaction starts to modify the storage of the blockchain, those changes are permanent, even if the transaction would fail at a later time during runtime execution.

This is necessary for blockchain systems since you may want to track things like the nonce of a user or subtract gas fees for any computation that occurred. Both of these things actually happen in the Ethereum state transition function for failed transactions, but you have never had to worry about managing those things as a contract developer.

Now that you are a Substrate runtime developer, you will have to be conscious of any changes you make to the state of your blockchain, and ensure that it follows the **"verify first, write last"** pattern. We will be helping you do this throughout the tutorial.

## Creating a List

Substrate does support lists in the form of an [`EnumerableStorageMap`](https://substrate.dev/rustdocs/v1.0/srml_support/storage/trait.EnumerableStorageMap.html), which is just a storage map like we have been using, but can be enumerated. From the docs:

> Note that type is primarily useful for off-chain computations. Runtime implementors should avoid enumerating storage entries.

In runtime development, list iteration is, generally speaking, dangerous. Unless explicitly guarded against, runtime functions which enumerate a list will add O(N) complexity, but only charge O(1) fees. As a result, your chain can be vulnerable to attacks. Furthermore, if the lists you iterate over are large or even unbounded, your runtime may need more time to process the list than what is allocated between blocks. This means that a block producer may not even be able to create new blocks!

For these reasons, this tutorial will not use any list iteration in our runtime logic. If you choose to include such logic, please keep the following in mind.

We will instead emulate an enumerable map with a mapping and a counter like so:

```rust
decl_storage! {
    trait Store for Module<T: Trait> as Example {
        AllPeopleArray get(person): map u32 => T::AccountId;
        AllPeopleCount get(num_of_people): u32;
    }
}
```

Here we are storing a list of people in our runtime represented by `AccountId`s. We just need to be careful to properly maintain these storage items to keep them accurate and up to date.

## Checking for Overflow/Underflow

If you have developed on Ethereum, you may be familiar with all the problems that can happen if you do not perform "safe math". Overflows and underflows are an easy way to cause your runtime to panic or for your storage to get messed up.

You must always be proactive about checking for possible runtime errors before you make changes to your storage state. Remember that unlike Ethereum, when a transaction fails, the state is NOT reverted back to before the transaction, so it is your responsibility to ensure that there are no side effects on error.

Fortunately, checking for these kinds of errors are quite simple in Rust where primitive number types have [`checked_add()`](https://doc.rust-lang.org/std/primitive.u32.html#method.checked_add) and [`checked_sub()`](https://doc.rust-lang.org/std/primitive.u32.html#method.checked_sub) functions.

Let's say we wanted to add an item to our `AllPeopleArray`, we should first check that we can successfully increment the `AllPeopleCount` like so:

```rust
let all_people_count = Self::num_of_people();

let new_all_people_count = all_people_count.checked_add(1).ok_or("Overflow adding a new person")?;
```

Using `ok_or` is the same as writing:

```rust
let new_all_people_count = match all_people_count.checked_add(1) {
    Some (c) => c,
    None => return Err("Overflow adding a new person"),
};
```

However, `ok_or` it is more is more clear and readable than `match`; you just need to make sure to remember the `?` at the end!

If we were successfully able to increment `AllPeopleCount` without an overflow, then it will simply assign the new value to `new_all_people_count`. If not, our module will return an `Err()` which can be gracefully handled by our runtime. The error message will also appear directly in our node's console output.

## Updating our List in Storage

Now that we have checked that we can safely increment our list, we can finally push changes to our storage. Remember that when you update your list, the "last index" of your list is one less than the count. For example, in a list with 2 items, the first item is index 0, and the second item is index 1.

A complete example of adding a new person to our list of people would look like:

```rust
fn add_person(origin, new_person: T::AccountId) -> Result {
    let sender = ensure_signed(origin)?;

    let all_people_count = Self::num_of_friends();
    
    let new_all_people_count = all_people_count.checked_add(1).ok_or("Overflow adding a new person")?;

    <AllPeopleArray<T>>::insert(all_people_count, new_person);
    <AllPeopleCount<T>>::put(new_all_people_count);

    Ok(())
}
```

We should probably add collision detection to this function too! Do you remember how to do that?

## Deleting From Our List

One problem that this `map` and `count` pattern introduces is holes in our list when we try to remove elements from the middle. Fortunately, the order of the list we want to manage in this tutorial is not important, so we can use a "swap and pop" method to efficiently mitigate this issue.

The "swap and pop" method switches the position of the item we want to remove and the last item in our list. Then, we can simply remove the last item without introducing any holes to our list.

Rather than run a loop to find the index of the item we want to remove each time we remove an item, we will use a little extra storage to keep track of each item and its position in our list.

We won't introduce the logic for "swap and pop" until later, but we will ask you to start tracking the index of each item using an `Index` storage like this example:

```rust
AllPeopleIndex: map T::AccountId => u32;
```

This is really just an inverse of the content in `AllPeopleArray`. Note that we do not need a getter function here since this storage item is used internally, and does not need to be exposed as part of our modules API.

## Your Turn!

Now that you know what is necessary to keep track of all of your kitties, let's update your runtime to do this.

Follow the template to add new storage items, and update your `create_kitty` function to **"verify first, write last"** whenever a new kitty is created.

<!-- tabs:start -->

#### ** Template **

[embedded-code](./assets/2.3-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](./assets/2.3-finished-code.rs ':include :type=code embed-final')

#### ** Previous Chapter Solution **

[embedded-code-previous](./assets/2.2-finished-code.rs ':include :type=code embed-previous')

<!-- tabs:end -->

---
**Learn More**

Talk about cloning and borrowing...

[TODO: make this a page]

---