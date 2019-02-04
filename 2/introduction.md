Introduction
===

[CryptoKitties](https://www.cryptokitties.co/) is a popular Ethereum dApp that at one point accounted for more than 20% of all incoming transactions on the Ethereum blockchain. In this tutorial, we will try to follow in their footsteps and create the next viral application on Substrate.

In this section, we will show you how to create a runtime which allows users create and own non-fungible tokens.

## Non-Fungible Token

CryptoKitties and other similar dApps use *non-fungible tokens* to represent their assets. A non-fungible token (NFT) is a token which is unique and distinguishable in nature. Whereas you can trade your Bitcoin for someone elses Bitcoin without really changing anything, when you create a kitty on CryptoKitties, you are generating a one of a kind, unique and irreplaceable item.

NFTs are particularly exciting not just because they are unique, but because of the things you can do with them! You can own a token, trade your tokens, buy or sell tokens, track the specific history of a token, and even have tokens interact with one another.

## ERC-721

[ERC-721](http://erc721.org/) is an extremely popular Ethereum token standard for non-fungible tokens. Since this standard is widely used, and there have been numerous smart contracts built using this standard, we will be using it as a rough basis for how our app should be structured.

Specifically, we will look to the [OpenZeppelin implementation](https://github.com/OpenZeppelin/openzeppelin-solidity/blob/master/contracts/token/ERC721/ERC721.sol), avoiding some of the more unnecessary features like "token approvals".

