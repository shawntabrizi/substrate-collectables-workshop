# Transfer Logic

Now that we scaffolded the `transfer` extrinsic, we can actually populate the appropriate logic to actually do a transfer.

## Sanity Checks

Before we should write any logic which actually transfers a kitty, we should sanity check that all the conditions are met for us to be able to actually execute the transfer.

### Don't Transfer to Yourself

The `do_transfer` logic has a `from` and `to` account. If they are the same, well we wouldn't really be doing a transfer.

As the developer, you can treat this as a `noop`, and return early, or as an error and return an error. In our pallet, we are choosing to emit an error.

This should be the first check you do because it takes no additional logic to make this check. You already have access to `from` and `to`, so checking equality is about the lightest amount of logic you could do.

Subsequent sanity checks will require us to read storage, and this is much more expensive to execute. If we can error early and without doing that storage read, that is way better for the blockchain.

So remember, error early, error fast.

### Does the Kitty Exist?

The input to the `transfer` extrinsic allows the sender to submit any `[u8; 32]` identifier to transfer, but we shouldn't assume that kitty actually exists in storage.

If the sender is trying to send a kitty which doesn't exist, we should emit an error.

That should be easy to write with something like:

```rust
let kitty = Kitties::<T>::get(kitty_id).ok_or(Error::<T>::NoKitty)?;
```

### Correct Owner?

This is an important one.

It could be super simple to really mess up your blockchain if you do not check that the right owner is the one who is actually initiating the transfer.

Now that we have the `kitty` object, we want to make sure the `from` account matches the `kitty.owner` account, else someone is trying to transfer the kitty who is not allowed to!

## Updating the Owner

At this point, we have done all of the sanity checks, and we can actually update the owner of the kitty. Based on our storage, we need to do this in three places:

- Update the `Kitty` object so that `kitty.owner` is set to `to`.
- Add the `kitty_id` from the vector of `KittiesOwned` for `to`.
- Remove the `kitty_id` from the vector of `KittiesOwned` for `from`.

This is really an exercise in writing Rust logic and using vector APIs. Unfortunately, we don't have any low level storage optimizations for such tasks, so you really just have to read both vectors and mutate them both.

There are still optimizations you can make by following the principle of fail early and fail fast. For example, it is probably better to try and add a new item to the `to_owned` bounded vector first to check that their vector isn't full. If it is full, we wouldn't be able to do the transfer anyway, so we could fail early and fast.

On the other hand, removing an item from a bounded vector can never fail, and since we already checked that `kitty.owner == from`, removing the `kitty_id` from `from_owned` should be infallible, unless there is really a big issue in the pallet logic. So the chances of an error here is much lower.

## Update Storage

Now that we have mutated the appropriate storage values, all that is left is to write those updated values back into storage.

No magic tricks here, just call the `insert` API.

The most important thing is to not forget to update everything!

## Your Turn

Follow the `TODO`s included in the template to flesh out the logic required to complete the `transfer` extrinsic.
