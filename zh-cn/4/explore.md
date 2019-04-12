# 探索 Substrate UI

Substrate UI 里面有很多内容。我们将尝试简化一些概念，使你能够开始定制你的 UI。

## Substrate UI 的基础部分

`substrate-ui` 模板预建了一般区块链 UI 中的这些功能。

- 用于管理和创建密钥 + 帐户的钱包
- 用于获取帐户详细信息的地址簿
- 在账户之间发送资金的转账功能
- 可以轻松更新 runtime 的 UX
- 修改 key/value 存储的 UX
- 自定义交易提交者

## React 组件

如果你进入 Substrate UI 代码，你将看到它是由 React Components 组成，每个 React Component 都有自己的用途。如上所述，Substrate UI 附带了一组预建功能，你可以在 `src` 文件夹中找到为这些功能提供支持的组件。

所有这些组件都放在 `src/app.jsx` 文件中，该文件为你看到的主视图提供支持。

## 读取我们的 Runtime

你可以看到 Substrate UI 可以访问我们的区块链。更令人印象深刻的是，它已经知道你创建的新 module，以及你添加到该 module 的特性和功能。让我们从 Substrate UI 的角度探索一些可以查看 module 的方法。

为此，在 Substrate UI 打开并运行的时候，我们将会在你的浏览器控制台中工作并使用自动补全。通常可以通过按 `F12` 或进入 `Inspect Element` 模式来访问它。

如果我们进入控制台并输入（注意末尾的点）：

```
runtime.
```

我们应该看到浏览器自动补全的选项列表：

![An image of the runtime autocomplete](../../4/assets/runtime-autocomplete.png)

你可能会注意到这些是我们的 modules！更好的是，我们可以看到 `substratekitties`。让我们深入进去看看。

```
runtime.substratekitties.
```

![An image of the substratekitties autocomplete](../../4/assets/runtime-substratekitties-autocomplete.png)

在这里，我们可以看到我们 modules 中公开的所有存储项。让我们读取其中一个：

![An image of querying the storage from browser console](../../4/assets/storage-from-browser.png)

```javascript
> runtime.substratekitties.allKittiesCount.then(console.log)

8

> runtime.substratekitties.allKittiesArray(0).then(console.log)

Hash(32) [58, 60, 214, 1, 126, 230, 54, 236, 38, 35, 250, 236, 81, 248, 64, 83, 234, 152, 174, 39, 114, 24, 108, 34, 128, 61, 74, 136, 74, 38, 206, 48]

> runtime.substratekitties.allKittiesArray(7).then(console.log)

Hash(32) [162, 202, 153, 236, 47, 9, 134, 176, 171, 201, 222, 149, 39, 69, 7, 46, 241, 155, 195, 52, 211, 62, 170, 24, 130, 50, 252, 36, 126, 209, 153, 38]
```

你可以看到此示例 runtime 有 8 个 kitties，你可以从 `allKittiesArray` 访问所有 kitties。（注意，该元素是 `kitty_id` hash，其中包含 32 个元素，范围为 0-255（32 个元素 * 8 bits = 256 bit hash）。）

## 在 Substrate UI 中使用 Runtime 变量

我们可以从简单开始，并尝试更新 Substrate UI 页面上包含的 live kitty 计数。

我们上面显示的变量是 Bonds，这意味着当区块链发生变化时，它们的值会自动更新。但是，这也意味着你去使用这些变量时，会与正常情况下有点不同（请注意，我们在上面的示例中使用了 JavaScript Promises）。

幸运的是，对于我们的示例，我们将使用为我们预构建的一些组件。在这种情况下，我们将使用名为 `Pretty` 的组件，它几乎可以将任何 bond 转换为可读字符串。你可以看到它正在被页面中的其他部分使用。

在我们的示例区块链中：

```
<Pretty value={runtime.substratekitties.allKittiesCount}/>
```

会转换成：

```
<span>8</span>
```

此外，随着 `allKittiesCount` 值的更改，我们的 HTML 将自动更新以表示最新值。这要归功于 React，它会自动重新呈现组件的内容。

## 轮到你了！

在模板中，我们目前有一个 `Subheader`：

```javascript
<Header.Subheader>There are 0 kitties purring.</Header.Subheader>
```

更新 `0` 值来呈现我们 runtime 中被追踪 kitties 的当前数量。然后使用 Polkadot UI 增加一些 kitties 并确认被自动更新的值。

<!-- tabs:start -->

#### ** Template **

[embedded-code](../../4/assets/4.2-template.js ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](../../4/assets/4.2-finished-code.js ':include :type=code embed-final')

<!-- tabs:end -->
