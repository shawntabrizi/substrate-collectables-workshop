# Set Price Logic

The set price logic is very straight forward.

The only thing we really need to check is that the `kitty.owner` matches the `caller` of the function.

Besides that, we simply need to update the `kitty.price` field, and write that back to the runtime storage.

## Your Turn

Follow the `TODO`s listed in the template, and complete the logic for the `do_set_price` function.
