# 开始玩我们的游戏

如果你已经完成到这一步了，那么 **恭喜你**！

你现在已经完成了 Substratekitties runtime module 的开发。如果你仔细按照我们的说明进行了所有正确的检查，那么你的代码应该与我们在此处显示的内容类似。

现在是使用 Polkadot-JS Apps UI 检查成果的好时机，确保你没有遇到任何错误或引入任何问题。

## 手动测试

你应该运行以下手动测试：

- 使用 token 为多个用户提供资金，以便他们都可以参与
- 让每个用户创建多个 kitties
- 通过使用正确和错误的所有者，尝试将 kitty 从一个用户转移给另一个用户
- 通过使用正确和错误的所有者，尝试设置 kitty 的价格
- 使用所有者和其他用户购买 kitty
- 使用不足的资金购买 kitty
- 高价购买 kitty，确保余额被适当减少
- 培育 kitty，检查新 DNA 是新旧混合物
- 完成所有这些操作后，确认所有用户都具有正确数量的 kitty，确认 kitty 总数正确，并且所有其他存储变量都被正确表示

## 挑战

我们向读者提出的挑战是扩展 Substratekitties runtime，并包含你可能希望在此 runtime module 中看到的其他功能和特性。

以下是一些想法：

- 在调用 `breed_kitty()` 期间追踪 kitty 的父母。也许只是一个 event...？

- 限制 `create_kitty()` 可以创建的 kitties 数量，添加价格曲线以使得创建每个新的 kitty 成本更高。

- 培育出新 kitty 后，当有人收到这只新 kitty，必须支付一定的费用。确保资金正确发送给每个用户。确保每个人都可以使用自己的 kitty 进行培育。

- 添加一个 kitty "fight"，使得两个 kitties 可以参与基于随机数和一些加权统计的游戏。胜利的 kitty 的数据有所增加，这使他们有更好的机会赢得下一轮比赛。

- 添加基因突变算法到 kitty 培育中，这将引入父母 kitties 都没有的特征。

- 为那些不想简单地设定定价以购买 kitty 的所有者推出拍卖系统。如果没有人投标则一段时间后拍卖结束。

你有什么其他想法吗？

在下一节中，我们将使用 Substrate JavaScript 库为此游戏创建自定义 UI。此自定义 UI 对命名的更改很敏感，因此你可能需要复制最终的 Substratekitties 代码以确保兼容性。

<!-- tabs:start -->

#### ** Solution **

[embedded-code-final](../../3/assets/3.5-finished-code.rs ':include :type=code embed-final')

<!-- tabs:end -->
