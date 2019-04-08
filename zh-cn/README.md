# [Substrate Collectables][main link]

> 使用 [Substrate][] 交互式地构建你的第一条区块链

![A screenshot of Substrate kitties](../media/substrate-collectables.png)

## 这是什么？

这是一个交互式的自我实践项目，你能学到如何使用 [Substrate][] —— 由 [Parity][] 开源的 [Rust][] 区块链开发组件，构建你的第一条区块链。通过这个实践项目，你将构建一条能创建资产，可交互的和管理其所有权的区块链。

该材料将着重于构建所述链的逻辑。它不会涉及区块链的网络，共识和经济激励方面。幸运的是，Substrate 内置了不错的网络和共识引擎，这使得我们可以专注于链的逻辑。

Substrate 是使用 Rust 构建的，Rust 是一种现代静态类型的系统编程语言。我们不会在该项目里详细介绍该语言的细节。Rust 语言非常容易阅读，如果你之前有过编程经验，那么即使你不熟悉 Rust，也不会在完成练习时遇到太多麻烦。[**译者注**: 我并不同意 “有过编程经验的开发者在编写 Rust 时不会遇到太多麻烦” 这种观点，我认为需要至少有过 C++/Haskell 的编程经验者才比较容易上手 Rust，因为 Rust 从 C++，Haskell 和其他编程语言里借鉴了很多优秀的部分，并基于这些构建了属于自身的独有特点]

## 怎么做？

只需逐章阅读材料，并完成每章的练习。虽然这些材料是为了让你能够独立完成，但我们强烈建议你与学习小组里的其他人一起合作。不时陷入困境或不理解材料中所解释的内容是完全正常的。在这时候，与周围的其他人交流会对问题的解决很有帮助。如果你对 [该材料有任何意见和问题，请反馈给我们][feedback]，对此我们会十分感谢。

# [Let's go!](zh-cn/0/introduction.md)

> **注意**
>
> Substrate 是一个快速发展的项目，这意味着在尝试遵循这些说明时，breaking changes 可能会导致你遇到问题。如果你遇到任何问题，请随时[与我们联系](https://substrate.readme.io/v1.0.0/docs/feedback)。

## 如何贡献

* 打开 [issues](https://github.com/shawntabrizi/substrate-collectables-workshop/issues)

* Fork 并且 clone [仓库](https://github.com/shawntabrizi/substrate-collectables-workshop)

* 为 [Visual Studio Code](https://code.visualstudio.com/) 启动一个本地的开发服务器，该服务器需要有比如  [Live Server](https://marketplace.visualstudio.com/items?itemName=ritwickdey.LiveServer) 的工具。

* 通过提交和推送改变到你的 fork 仓库，并且创建一个 Pull Request 给上游仓库。

## 致谢

如果没有开发社区的协作努力，像 Substrate 这样的开源项目和这个 workshop 就无法成功。

Substratekitties workshop 是在 [Cryptokitties](https://www.cryptokitties.co/), [Cryptozombies](https://cryptozombies.io/), [Docsify](https://docsify.js.org/), [Monaco Editor](https://microsoft.github.io/monaco-editor/), [David Revoy's Cat Avatar Generator](https://framagit.org/Deevad/cat-avatar-generator) 以及众多志愿者的贡献的基础上完成的。

我们希望这种材料可以教会你一些新的东西，反过来，你也教会别人一些新的东西。

---

[main link]: https://shawntabrizi.github.io/substrate-collectables-workshop/
[feedback]: https://docs.substrate.dev/docs/feedback
[Substrate]: https://www.parity.io/substrate/
[Substrate docs]: https://docs.substrate.dev/
[Parity]: https://www.parity.io/
[Rust]: https://www.rust-lang.org/
