# Introduction

Welcome to the new Substrate Collectables Workshop.

The goal of this tutorial is to **teach by experience** various entry level concepts around Polkadot Pallet development.

Before we get started writing code, we will need to cover some basics.

## Prerequisites

The tutorial is designed to be completed by anyone with basic familiarity with [Rust](https://www.rust-lang.org/), and little to no familiarity with the [Polkadot SDK](https://github.com/paritytech/polkadot-sdk).

If you do not feel comfortable with the level of Rust used in this tutorial, we recommend you first check out the [`rust-state-machine`](https://github.com/shawntabrizi/rust-state-machine) tutorial.

## Tutorial Structure

This tutorial is broken up into small steps with documentation and code.

Documentation should teach you all the concepts needed to complete the tutorial, while the code will ensure that you can actually execute on the knowledge gained.

The code in each step should have comments marked `TODO`, which indicate the actions you need to take in order to complete the step successfully.

At each step, we include a `diff` view so that you can see what has changed from the last step (new action items), and what should change to complete the step (the solution).

At the end of each step, you should be able to run all the following commands without any errors or warnings:

- `cargo +nightly fmt`
- `cargo +nightly clippy`
- `cargo test`

We recommend you run these during each step in the tutorial to confirm you are doing everything correctly.

## Clone the Starting Template

The best way to follow this tutorial is to clone the starting template, and follow the steps of the tutorial locally.

Run the following:

```bash
git clone https://github.com/shawntabrizi/substrate-collectables-workshop/ -b starting-template --single-branch
cd substrate-collectables-workshop
```

Or access the template directly here:

https://github.com/shawntabrizi/substrate-collectables-workshop/releases/tag/starting-template

We will go over the starting template in detail throughout this section, but don't be afraid to peek around yourself.

Make sure your checks pass on this initial template without warning or errors:

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
