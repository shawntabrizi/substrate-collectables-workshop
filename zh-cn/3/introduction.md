# 介绍

此时，你已经了解了 Substrate runtime 开发的大部分核心内容。但是，我们的 runtime 不是很有趣。

下一节将教你如何向 runtime 添加其他功能，以启用原始 CryptoKitties 游戏的一些最流行的功能：

- 转让 kitty
- 买一只 kitty
- 养一只 kitty

要启用这些功能，我们需要与 SRML 中的其他 modules 进行交互，比如 `Balances` module。我们还将继续使用 Substrate 特定类型进行更多工作。
