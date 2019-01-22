Transferring a Kitty
===

Now that we have enabled users to own multiple kitties, we may also want to add functionality which allows one user to transfer a kitty they own to another user.

Ownership is entirely managed by our storage, so a `tranfer_kitty` function is really only modifying our existing storage to reflect the new state.

Let's think about the storage items we need to update:

 - Change the global token owner
 - Change the owned token count of each user
 - Change the owned token index of the kitty
 - Change the owned token map for each user

Again, we will be following the "**check, then store**" pattern, and produce a function that looks like:

[TODO: Make users build these functions rather than giving them code]

```

```