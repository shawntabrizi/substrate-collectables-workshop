# [Substrate Collectables][main link]

> 使用 [Substrate][] 交互式地构建你的第一条区块链

[演示PPT](https://docs.google.com/presentation/d/1dhaoLb5V2K_vDe4EJlUcKwePD1nMktr57fOdSo8bHns/edit#slide=id.g45ee0ba2ab_3_12)

![A screenshot of Substrate kitties](../media/substrate-collectables.png)

## 这是什么？

这是一个交互式的自我实践项目，你能学到如何使用 [Substrate][] —— 由 [Parity][] 开源的 [Rust][] 区块链开发工具，构建你的第一条区块链。通过这个实践项目，你将构建一条收藏品区块链应用，能创建资产，可与之交互和管理其所有权。

本文将着重于构建上述区块链的逻辑。它不会涉及区块链的网络，共识和经济激励方面。幸运的是，Substrate 内置了不错的网络和共识引擎，这使得我们可以专注于链的逻辑。

Substrate 是使用 [Rust][] 构建的，Rust 是一种现代静态类型的系统编程语言。我们不会在这次工作坊里详细介绍该语言的细节。Rust 语言十分易读，如果你之前有过编程经验，即使你不熟悉 Rust，也不会在完成练习的过程中遇到太多麻烦。

## 怎么做？

只需逐章阅读材料，并完成每章的练习。虽然这些材料是为了让你能够独立完成，但我们强烈建议你参与学习小组或组织的工作坊，与其他人一起合作。不时地遇到困难或不理解材料中所解释的内容是完全正常的。在这时候，与周围的其他人交流会对问题的解决很有帮助。如果你对 [该材料有任何意见和问题，请反馈给我们][feedback]，对此我们会十分感谢。

# [Let's go!](zh-cn/0/introduction.md)

> **注意**
>
> Substrate 是一个快速发展的项目，这意味着在尝试遵循这些说明时，breaking changes 可能会导致你遇到问题。如果你遇到任何问题，请随时[与我们联系](https://substrate.readme.io/v1.0.0/docs/feedback)。

## 如何贡献

* 提 [issues](https://github.com/substrate-developer-hub/substrate-collectables-workshop/issues)

* Fork 并且 clone [仓库](https://github.com/substrate-developer-hub/substrate-collectables-workshop)

* 在本地服务器上启动 [Visual Studio Code](https://code.visualstudio.com/), 并且安装有类似 [Live Server](https://marketplace.visualstudio.com/items?itemName=ritwickdey.LiveServer) 的扩展工具。

* 通过提交和推送修改到你的 fork 仓库，接着创建一个 Pull Request 给上游仓库。

## 致谢

如果没有社区的集思广益和协作努力，像 Substrate 这样的开源项目和这个 workshop 是无法成功完成的。

Substratekitties 工作坊是在 [Cryptokitties](https://www.cryptokitties.co/), [Cryptozombies](https://cryptozombies.io/), [Docsify](https://docsify.js.org/), [Monaco Editor](https://microsoft.github.io/monaco-editor/), [David Revoy's Cat Avatar Generator](https://framagit.org/Deevad/cat-avatar-generator) 以及众多志愿者的贡献的基础上完成的。

我们希望这份材料可以教会你一些新的东西，反过来，也希望你将这些东西教给他人。

---

[main link]: https://substrate-developer-hub.github.io/substrate-collectables-workshop/
[feedback]: https://substrate.dev/community/
[Substrate]: https://www.parity.io/substrate/
[Substrate docs]: https://substrate.dev/docs/
[Parity]: https://www.parity.io/
[Rust]: https://www.rust-lang.org/
