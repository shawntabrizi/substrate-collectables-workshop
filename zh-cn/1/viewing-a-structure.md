# 查看 Structure

现在我们已经为 kitty 构建好了 runtime，现在需要检查一下我们的工作！

我们已经为我们的链引入了一个自定义结构，虽然 Polkadot-JS Apps UI 非常善于适应我们的变化，但在这种情况下，我们需要给出一个关于如何反序列化结构体数据的提示。

> 提示：请记住重置你的链，以便在与 UI 交互时重新开始：
>
> ```bash
> ./scripts/build.sh
> cargo build --release
> ./target/release/substratekitties purge-chain --dev
> ./target/release/substratekitties --dev
> ```

## 注册一个自定义 Struct

幸运的是，Polkadot-JS Apps UI 为我们提供了一种非常简单的方法来导入自定义结构，以便页面能够正确解码信息。

在 **Settings** app 页面的 **Developer** 部分中，你可以提交包含有自定义 struct 的 JSON 文件或者通过代码编辑器手动添加。将此JSON object 复制并粘贴到代码编辑器中，然后按下 `Save`。

```json
{
    "Kitty": {
        "id": "H256",
        "dna": "H256",
        "price": "Balance",
        "gen": "u64"
    }
}
```

## 创建一个 Kitty

现在我们可以去创造一个新的 kitty。在 **Extrinsics** app 页面中，进入：

```
substratekitties > createKitty()
```

一旦你按下提交后，你应该能看到交易完成：

![Image of creating a kitty in the Polkadot-JS Apps UI](../../1/assets/creating-a-kitty.png)

## 查看 Kitty

最后，我们可以进入 **Chain State** app 页面，查看我们存储的 kitty 对象。选择：

```
kittyStorage > ownedKitty(AccountId): Kitty
```

接着选择已调用 `createKitty()` 函数的用户。然后，你应该能够看到 `Kitty` 对象的各个属性：

![Image of viewing a kitty object in the Polkadot UI](../../1/assets/view-kitty.png)

---

**Learn More**

聊聊序列化和反序列化。

我们如何简单地传输原始字节

[TODO: make this a page]

---
