# Substrate Collectables Workshop

A guided, hands-on tutorial for building a simple NFT marketplace pallet with the [Polkadot SDK](https://github.com/paritytech/polkadot-sdk).

<img src="./screenshot.png" width="400px" />

## Read the Tutorial

The tutorial is published as an interactive book:

**👉 https://www.shawntabrizi.com/substrate-collectables-workshop/**

That is the recommended way to follow along. This repository is the source.

## Goal

Teach entry-level Polkadot pallet development **by experience**. By the end you will have written a custom pallet that mints, transfers, and trades collectable "kitties" — an on-chain NFT marketplace.

## Who It's For

Anyone with basic [Rust](https://www.rust-lang.org/) familiarity and little to no Polkadot SDK background.

If you are not yet comfortable with the Rust used here, start with the [`rust-state-machine`](https://github.com/shawntabrizi/rust-state-machine) tutorial first.

## Repository Layout

This repo has two branches that matter for contributors:

- [`master`](https://github.com/shawntabrizi/substrate-collectables-workshop/) — the mdBook source for the published tutorial. Each chapter lives under `src/<N>/`. **Start here for small fixes.**
- [`gitorial`](https://github.com/shawntabrizi/substrate-collectables-workshop/tree/gitorial) — the same content repackaged as a linear Git history, one commit per step. Regenerated automatically from `master`.

## Contributing

For typos or changes scoped to a single step, open an issue or PR against `master`.

For changes that span multiple steps, the `gitorial` branch plus the [`gitorial-cli`](https://github.com/gitorial-sdk/cli) tooling are the right tools. The Gitorial format, commit conventions, and editing workflow live at **[gitorial-sdk](https://github.com/gitorial-sdk)**.

> Looking for the original 2020 version of this tutorial? It lives on the [`docsify-old` branch](https://github.com/shawntabrizi/substrate-collectables-workshop/tree/docsify-old). The content there is out of date — this rewrite supersedes it.
