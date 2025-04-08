# Transfer Extrinsic

We will build the `transfer` extrinsic over two steps. In this step, we will just step up the skeleton of the extrinsic, the internal function, and the event that will be emitted at the end of the extrinsic.

## Clean Code

It's been a while since we started programming using the initial template provided by this tutorial.

In that template, we already scaffolded for you the `create_kitty` extrinsic, the `mint` internal function, and the `Created` event.

In this step, we will be doing the same thing, but for a new extrinsic `transfer`.

To keep our code clean, we put a minimal amount of logic inside the `transfer` extrinsic, and instead push most of our logic into a `do_transfer` internal function.

Remember that things which rely on the `#[pallet::*]` macros must be in the same file.

But if you look closely, all of the "internal" functions like `gen_dna`, `mint`, and soon `do_transfer`, do not depend on the `#[pallet::*]` macros at all!

So you could actually move all of this logic into its own file, and that would lead to much cleaner code. The same thing can be said for the definition of the `Kitty` struct and any other custom types you might use in your Pallet.

However, for the purposes of this tutorial, keeping everything in one file makes things a bit easier to teach.

## Your Turn

There is nothing in this step that you are not already familiar with.

Follow the `TODO`s in the template to:

- Create a `transfer` extrinsic.
- Create a `do_transfer` internal function.
- Create a `Transferred` event.
