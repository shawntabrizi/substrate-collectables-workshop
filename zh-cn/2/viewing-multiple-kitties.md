# 查看多个 Kitties

自从我们上次使用 Polkadot UI 以来，我们在 runtime 存储中添加了很多项，而且我们的存储结构也变得有点复杂了。现在是时候来检查下我们到目前为止所做的工作，并了解一下我们如何查看在 runtime 生成的 kitties。

在进入 UI 之前，请记住启动一条全新的链：

```bash
./scripts/build.sh
cargo build --release
./target/release/substratekitties purge-chain --dev
./target/release/substratekitties --dev
```

## 使用多个账户

你要做的第一件事是将资金从 Alice 转移到一些已经被提供的帐户里。让我们向 Bob 和 Charlie 汇款吧。

现在我们将进入 **Extrinsics** 页面，在 UI 中选择我们的 `create_kitty()` 函数：

```
substratekitties > createKitty()
```

在我们的测试中，我们将让 Alice 创建 3 个 kitties，让 Bob 创建 2 个 kitties，让 Charlie 创建 1 个 kitty。

![An image of Alice creating a Kitty](../../2/assets/alice-creates-kitty.png)

## 查看我们的存储

现在我们可以浏览我们设置的所有存储项，确保一切正常。

首先，我们应该检查系统中的 kitties 总数：

```
kittyStorage > allKittiesCount(): u64
```

如果一切顺利，我们应该得到返回值 6 。

![An image of AllKittiesCount](../../2/assets/all-kitties-count.png)

接下来我们应该检查每个用户的 kitty 数量，Alice 为 3，Bob 为 2，Charlie 为 1：

![An image of individual kitty count](../../2/assets/owner-kitty-count.png)

如果我们看下 `AllKittiesArray`，我们应该能够通过它们的全局索引获得每个 kitty。我们看下第五个 kitty (index 为 4)，并确认所有者是 Bob。此外，我们可以确认这是 Bob 的第 2 个 kitty，因为相对的 `OwnedKittiesIndex` 是 `1`。

![An image of Bob's kitty in storage](../../2/assets/bob-owned-kitty.png)

## 轮到你了！

还有一些我们尚未检查的存储项。花一些时间来确保一切正常。如果发现错误，请回到 runtime 以查看是否可以找出问题所在。
