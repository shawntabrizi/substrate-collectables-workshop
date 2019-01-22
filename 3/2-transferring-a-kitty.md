Transferring a Kitty
===

Now that we have enabled users to own multiple kitties, we may also want to add functionality which allows one user to transfer a kitty they own to another user.

Ownership is entirely managed by our storage, so a `tranfer_kitty` function is really only modifying our existing storage to reflect the new state.

Let's think about the storage items we need to update:

 - Change the global token owner
 - Change the owned token count of each user
 - Change the owned token index of the kitty
 - Change the owned token map for each user

 ## Swap and Pop

 We mentioned earlier in this tutorial that we will be using a "swap and pop" method to remove items from our makeshift lists. It is important to remember that this method causes us to change the order of list, but otherwise is pretty efficent.

 If you are building a runtime where order of your list matters, you will need to rethink how you will manage this, but this is not an algorithms course, so we will provide you with working code for your runtime.

 ```
let kitty_index = <OwnedKittiesIndex<T>>::get(kitty_id);

if kitty_index != new_owned_kitty_count_from {
    let last_kitty_id = <OwnedKittiesArray<T>>::get((from.clone(), new_owned_kitty_count_from));
    <OwnedKittiesArray<T>>::insert((from.clone(), kitty_index), last_kitty_id);
    <OwnedKittiesIndex<T>>::insert(last_kitty_id, kitty_index);
}
```

Note that we have included a small optimization which only does a "swap" if the index we want to remove is not already the last element in the array. In that situation, only a "pop" is needed.

## Your Turn

To do a transfer, you will need to use the tools you have learned so far, but nothing new.

We want to make the transfer function reusable, so we have already structured the template to help you here.

Follow the template to complete the `_transfer_from()` private function to power your public `transfer()` function exposed by your module.