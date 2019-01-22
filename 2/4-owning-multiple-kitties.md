Owning Multiple Kitties
===

Our naive impementation of kitty creation allowed each user to have a single kitty to their name. But if we want to really build and extend our game, we need to support users each having and managing multiple kitties.

Do enable this, we will introduce our final set of storage items used for tracking multiple kitties per owner.

## Using Tuples to Emulate Higher Order Arrays

We are going to need to introduce some more complex storage items to represent ownership of multiple items across multiple users.

Fortunately, for our needs, using a [tuple](https://doc.rust-lang.org/rust-by-example/primitives/tuples.html) of `AccountId` and `Index` can pretty much solve our problems here.

Here is how we could build a "friends list" unique to each person using such a structure:

```
MyFriendsArray get(my_friends_array): map (T::AccountId, u32) => T::AccountId;
MyFriendsCount get(my_friends_count): map T::AccountId => u32;
```

This should emulate a more standard two-dimensional array like:

```
MyFriendsArray[AccountId][Index] -> AccountId
```

and getting the number of friends for a user like:

```
MyFriendsArray[AccountId].length()
```

## Relative Index

Just as before, we can optimize the computational work our runtime needs to do by indexing the location of items. The general approach to this would be to reverse the mapping of `MyFriendsArray`, and create a storage item like this:

```
MyFriendsIndex: map (T::AccountId, T::AccountId) => u32;
```

Where the `AccountId`s wold represent the user, their friend, and the returned value would be the index of `MyFriendsArray` for that user where the friend is stored.

[TODO: May just cut this, and update the struct in our code ]
However, since our kitties all have unique identifiers as a `Hash`, and cannot be owned by more than one user, we can actually simplify this structure:

```
MyKittiesIndex: map T::Hash => u32;
```

This allows us to fetch the ...
[TODO: finish this if it is not removed]

## Your Turn!

[TODO: Add stuff]

Nothing here should be too surprising... its just like doing some taxes.

---
**Learn More**

Talk about Option<T::AccountID>

[TODO: make this a page]

---