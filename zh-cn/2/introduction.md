# 介绍

[CryptoKitties](https://www.cryptokitties.co/) 是一款流行的 Ethereum dApp，它曾经在以太坊区块链的所有发起交易中占据了20％以上的份额。在本教程中，我们将尝试跟随他们的脚步并在 Substrate上创建下一个如病毒传播式的应用程序。

在本节中，我们将向你展示如何创建一个 runtime, 允许用户创建和拥有不可替代通证。

## 不可替代通证

CryptoKitties 和其他类似的 dApp 使用 *不可替代通证* 来表示其资产。不可替代通证（NFT）是一种独特且可区分的 token。尽管你可以在没有真正改变任何东西的情况下与他人交易 Bitcoin，但是​​当你在 CryptoKitties 上创建 kitty 时，你将生成一种独特且不可替代的物品。

NFTs 的出现特别令人兴奋，不仅仅是因为它们是独一无二的，而是因为你可以用它们所能做的事情！ 你可以拥有 token，交易 token，买入或卖出 token，追踪 token 的特定历史记录，甚至可以让 token 彼此交互。

## ERC-721

[ERC-721](http://erc721.org/) 是一项非常受欢迎的 Ethereum token 标准，用来表示不可替代通证。由于该标准被广泛使用，并且有许多智能合约使用该标准构建，所以我们会使用它作为构建我们的应用程序的基础。

具体来说，我们将参考 [OpenZeppelin 实现](https://github.com/OpenZeppelin/openzeppelin-solidity/blob/master/contracts/token/ERC721/ERC721.sol)，避免一些不必要的功能，如 “token approvals”。
