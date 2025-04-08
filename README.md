# Setup

Before we start writing code, we will need to setup your computer for this tutorial.

## Prerequisites

The tutorial is designed to be completed by anyone with basic familiarity with [Rust](https://www.rust-lang.org/), and little to no familiarity with the [Polkadot SDK](https://github.com/paritytech/polkadot-sdk).

If you do not feel comfortable with the level of Rust used in this tutorial, we recommend you first check out the [`rust-state-machine`](https://github.com/shawntabrizi/rust-state-machine) tutorial.

## Goal

The goal of this tutorial is to create a custom Polkadot SDK Pallet that acts as an NFT Marketplace. Our NFTs will represent kitties, which will be a digital pets that can be created, traded, and more.

This Pallet could then be included into a Polkadot SDK blockchain and used to launch a Web3 application on the Polkadot Network.

TODO: need to create and link to a tutorial creating and launching a custom runtime with omninode.

## Tutorial Structure

This tutorial is broken up into small steps with documentation and code.

Documentation should teach you all the concepts needed to complete the tutorial, while the code will ensure that you can actually execute on the knowledge gained.

The code in each step should have comments marked `TODO`, which indicate the actions you need to take in order to complete the step successfully.

At each step, we include a `diff` view so that you can see what has changed from the last step (new action items), and what should change to complete the step (the solution).

At the end of each step, you should be able to run all the following commands without any errors or warnings on the latest version of Rust (`rustup update`):

- `cargo +nightly fmt`
- `cargo +nightly clippy`
- `cargo test`

We recommend you run these during each step in the tutorial to confirm you are doing everything correctly.

### Tests

Included in this project is a `tests.rs` file which is run when you execute `cargo test`.

As we build out the project, we will include more tests to verify your logic is working as expected.

This tutorial does not really go into details about writing tests for Pallets, but it is something you probably can learn and understand by looking at the tests included here.

You are welcome to dive deeper into the included tests provided by the solution, or you can feel free to just copy and paste the latest `tests.rs` file into your project and make sure that everything is passing.

At some points in the tutorial, we will make breaking changes to our pallet, for example updating types, structure, or function signatures.

Remember to update your tests for those step, otherwise they will incorrectly report that you have a bug in your code.

## Starting Template

The template for this project is a minimal starting point for developing a custom Pallet with a little bit of starting code.

### Clone

The best way to follow this tutorial is to clone the starting template, and follow the steps of the tutorial locally.

Run the following:

```bash
git clone https://github.com/shawntabrizi/substrate-collectables-workshop/ -b starting-template --single-branch
cd substrate-collectables-workshop
```

Or access the template directly here:

[https://github.com/shawntabrizi/substrate-collectables-workshop/releases/tag/starting-template](https://github.com/shawntabrizi/substrate-collectables-workshop/releases/tag/starting-template)

### Install

The starting template includes a `README` with instructions to setup your working environment. Follow those instructions.

Make sure your rust compiler is up to date with `rustup update`.

Then, make sure you are able to run the following checks on this starting template without warnings or errors:

```bash
cargo +nightly fmt
cargo +nightly clippy
cargo test
```

It may take a while for this to complete based on how powerful your computer is.

Feel free to move onto the next steps while these checks are compiling.

## Contribute

This tutorial is completely free to use, modify, and distribute in any way you see fit.

If you catch any problems with the tutorial or have ideas on how it can be improved, open an [issue](https://github.com/shawntabrizi/substrate-collectables-workshop/issues) or a [pull request](https://github.com/shawntabrizi/substrate-collectables-workshop/pulls).
