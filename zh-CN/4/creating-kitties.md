# 创建 Kitties

现在既然我们已经尝试了 bonds 并从 UI 访问 runtime 存储，那么让我们实际构建一些交互。

## 调用我们的 Runtime

在上一节中，我们研究了 Substrate UI 创建的 `runtime` 变量。现在让我们来看看 `calls` 变量：

```
calls.
```

![An image of the `calls` autocomplete](../../4/assets/calls-autocomplete.png)

我们再次看到，我们可以自动访问所有 modules，包括我们刚刚创建的 module。深入了解 `substratekitties`，我们发现：

```
calls.substratekitties.
```

![An image of `calls.substratekitties` autocomplete](../../4/assets/calls-substratekitties-autocomplete.png)

这是我们在 `decl_module!` 宏中创建的所有函数的列表。请注意，我们的私有函数如 `mint()` 和 `_transfer()` 不在此处，因为它们不应该是我们的公共 API 的一部分。

## 创建调用

所以我们试着创建一个调用来创建一个新的 kitty。为此，我们可以向 runtime 发出 `post()` 请求，如下所示：

```javascript
post({
    sender: '5GoKvZWG5ZPYL1WUovuHW3zJBWBP5eT8CbqjdRY4Q6iMaDtZ',
    call: calls.substratekitties.createKitty()
}).tie(console.log)
```

在此示例中，`sender` 是我们其中一个帐户的地址。我们可以在 Substrate UI 的 **Address Book** 中检索我们的所有账户：

![An image of the Address Book section](../../4/assets/address-book.png)

当我们在控制台中提交此内容时，我们会在后台看到一些事情发生，交易最终 `finalised` 并且 kitties 数量增加：

![An image of creating a kitty from console](../../4/assets/transaction-from-console.png)

## 创建一个交易按钮

现在我们知道如何调用我们的 runtime，我们希望将它集成到我们的 UX 中。同样，我们将利用 Substrate UI 提供给我们的名为 `TransactionButton` 和 `SignerBond` 的组件。

如果你查看页面上其他部分的代码，你将找到如何集成这些部分的示例。

`SignerBond` 创建一个输入区域，在里面每个人可以写入帐户名称，用于他们签署某些消息。该账户被置于 `bond` 内。

```javascript
this.account = new Bond;

<SignerBond bond={this.account}/>
```

然后你可以使用此 bond 为 `TransactButton` 提供支持，我们将使用存储在 bond 中的值为交易的 `sender` 字段提供支持：

```javascript
<TransactButton
    content='Submit Transaction'
    icon='send'
    tx={{
        sender: runtime.indices.tryIndex(this.account),
        call: calls.myModule.myFunction()
    }}
/>
```

因为 `TransactionButton` 依赖于 `this.account`，所以在 `SignerBond` 具有有效输入之前它不会处于活动状态。完成后，你可以代表该用户轻松提交交易。

## 其他组件

我们不会深入了解 Substrate UI 提供的每个组件。但是，这些组件中的大多数应该都相对容易理解并能在你自己的代码里重用。

除了 [React 组件](https://reactjs.org/docs/react-component.html) 中可见的内容之外，几乎没有什么神奇的事情发生，所以这将是一个你开始扩展知识的好时候。

## 轮到你了！

让我们在你的 Substrate UI 中添加一个 `Create Kitty` 按钮。

你需要在 `constructor()` 中创建一个新的 bond，并使用该 bond 为 `SignerBond` 组件提供支持。

然后，你需要将 `SignerBond` 连接到 `TransactionButton`，后者会调用 `createKitty()`。你可以使用 `paw` 图标为你的交易按钮增加一点额外的效果。

完成此操作后，通过创建一些新的 kitty 来测试你的按钮！当新的 kitty 进入你的系统时，观察你的 “kitty counter” 是否增加。

<!-- tabs:start -->

#### ** Template **

[embedded-code](../../4/assets/4.3-template.js ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](../../4/assets/4.3-finished-code.js ':include :type=code embed-final')

<!-- tabs:end -->
