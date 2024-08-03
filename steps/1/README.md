# Polkadot-SDK

Our starting template for this tutorial uses the [Polkadot SDK](https://github.com/paritytech/polkadot-sdk).

This is the same technology stack used to build and power the [Polkadot Network](https://polkadot.network/).

To better understand what you will be doing in this tutorial, we need to give you a high level overview of the blockchain ecosystem to make sure we are all on the same page.

## Blockchain

Blockchains are the foundation of building Web3 technologies.

Web3 is a promise toward a world with less trust, and more truth.

Through blockchains, people are able to develop and deploy software that are decentralized, open, permissionless, censorship resistant, and independently verifiable.

The main purpose of a blockchain node is to come to consensus with other nodes on the decentralized network.

If you want to learn more about blockchains, check out the following video from the Polkadot Blockchain Academy:

<iframe width="560" height="315" src="https://www.youtube.com/embed/8UvdfFGYFiE?si=5PIyppVBZ91vUtjf" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>

## Runtime

At the heart of a blockchain is a [state transition function](https://en.wikipedia.org/wiki/Finite-state_machine) (STF).

This is the logic of the blockchain, and defines all the ways a blockchain is allowed to manipulate the blockchain state.

In the `polkadot-sdk` we refer to this logic as the blockchain's runtime.

All nodes on a blockchain network have and use the same runtime, allowing them to come to consensus about changes to a blockchain.

To learn more about the runtime, and its role inside of the `polkadot-sdk`, check out this video from the Polkadot Blockchain Academy:

<iframe width="560" height="315" src="https://www.youtube.com/embed/-ttmm8gYS04?si=ZH_g83CVtguENoK7" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>

## Pallets

The `polkadot-sdk` provides a developer framework called FRAME.

FRAME is an opinionated framework on how one should quickly and easily build and maintain a blockchain's runtime.

> NOTE: It is important to clarify that FRAME is not the only way you can develop a runtime for the `polkadot-sdk`, but it is the one that the Polkadot Network uses, and as a result, is most supported by the ecosystem.

FRAME's key decision is to break apart the blockchain runtime into separate logical pieces that can choose to interact with one another.

These logical pieces are called Pallets.

TODO: Add images.

You can think of different Pallets as different applications or functions that your blockchain exposes.

You can also think of Pallets very similar to traditional blockchain smart contracts, however Pallets are more powerful and execute much faster than smart contracts.

To learn more about FRAME and Pallets, check out this video from the Polkadot Blockchain Academy:

<iframe width="560" height="315" src="https://www.youtube.com/embed/ghMloMzEEsA?si=3DtsmrYOapbnR2oy" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>

## NFTs

Non-Fungible Tokens (NFTs) are a type of token which can be created and traded on a blockchain.

As their name indicated, each NFT is totally unique, and therefore non-fungible with one another.

NFTs can be used for many things, for example:

- Representing real world assets
	- Ownership Rights
	- Access Rights
- Digital assets
	- Music
	- Images
	- Skins
	- Characters
- and much more...

## Starting Template

The template for this project is a minimal starting point for developing a custom Pallet.

In this tutorial, we will create a Pallet which allows us to create and manage a collection of NFTs.

Our NFTs will represent kitties, which will be a digital pet that can be created, traded, and more.

This Pallet could then be included into a `polkadot-sdk` project and used to launch a Web3 application on the Polkadot Network.

TODO: need to create and link to a tutorial creating and launching a runtime with omninode.
