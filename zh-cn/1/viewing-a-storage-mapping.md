# 在 Polkadot UI 中检查我们的工作

即使这段代码编译没有出错，现在也是检查我们工作的好时机。

运行以下命令后：

```bash
./scripts/build.sh
cargo build --release
./target/release/substratekitties purge-chain --dev
```

我们可以启动节点：

```bash
./target/release/substratekitties --dev
```

如果我们回到 [Polkadot-JS Apps UI](https://substrate-ui.parity.io/)，我们应该看到我们的节点正在产生块。

## 提交一个 Transaction

进入 **Extrinsics** app 页面，然后在 “from extrinsic section” 下拉列表中选择：

```
substratekitties > setValue(value)
```

输入值并按下 `Submit Transaction`：

![Submit a storage mapping in the Polkadot-JS Apps UI](../../1/assets/submit-storage-mapping.png)

## 查看存储

现在你已经提交了一个将值存入存储的交易，我们看看该值实际上存在与否。

转到 **Chain state** app 页面，然后选择：

```
substratekitties > value(AccountId): u64
```

在你提交交易的帐户中，查询存储并按下蓝色 `[+]` 按钮：

![Query for storage mapping](../../1/assets/view-storage-mapping.png)

你应该得到了与你存储的相同值！你可以尝试使用多个帐户，并查看每个用户是否能够将自己的值存储到 runtime 存储中。
