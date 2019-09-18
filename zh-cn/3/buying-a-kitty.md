# 购买 Kitty

现在我们可以设置 kitty 的价格并转让 kitty 的所有权，我们拥有构建 `buy_kitty` 函数所需的一切。

## 检查用于出售的 Kitty

在我们允许用户执行 `buy_kitty()` 函数之前，我们应该确保 kitty 确实可以出售。我们已经简化了我们的示例，任何默认价格为 0 的 Kitty 都不会被出售。所有者可以轻松地调用 `set_price()`， 设置他们 kitty 的价值为 0，并将其从市场上撤下。

你可以使用该类型公开的函数轻松检查 `T::Balance` 是否为零：

```rust
let my_value = <T::Balance as As<u64>>::sa(0);
ensure!(my_value.is_zero(), "Value is not zero");
// `ensure` will succeed and execution continues here
```

如果你想改善这一点，我们可以将价格定为 `Option<T::Balance>`，其中 0 将是有效价格，而由 `None` 表示不能出售... 但我们会将其当作挑战，留给读者自己实现。

## 进行付款

到目前为止，我们的链完全独立于 `Balances` module 提供的内部货币。`Balances` module 使我们能够完全管理每个用户的内部货币，这意味着我们需要谨慎使用它。

幸运的是，`Balances` module 暴露出了一个名为 `Currency` 的 trait, 它实现了一个叫 [`transfer()`](https://substrate.dev/rustdocs/v1.0/srml_support/traits/trait.Currency.html#tymethod.transfer) 的函数，它允许你安全地将 units 从一个帐户转移到另一个帐户，检查是否有足够的余额，是否上溢或下溢，甚至还能检查为获取 tokens 而创建的账户。

要访问 `Currency` trait 及其实现的所有函数，你需要将 trait 导入到模块中：

```rust
use support::traits::Currency;
```

然后，你可以使用以下语法访问此特定函数：

```rust
<balances::Module<T> as Currency<_>>::transfer(&from, &to, value)?;
```

## 记住："Verify First, Write Last"

如果你看一下`transfer（）`函数的实现，它既可以 "verifies" 也可以 "writes"，这意味着你需要小心谨慎地将它作为模块逻辑的一部分包含在内。

```rust
// end of verifications

<balances::Module<T> as Currency<_>>::transfer(&from, &to, value)?;

// beginning of writing to storage
```

如果你看过我们的模板，你会看到我们会建议你在调用 `transfer()` 之后直接执行 `transfer_from()` 函数。

注意到一个问题？

```rust
// nothing after this line should fail
<balances::Module<T> as Currency<_>>::transfer(&from, &to, value)?;
// but this function technically "could" fail
Self::transfer_from(owner.clone(), sender.clone(), kitty_id)?;
```

那么我们在这里打破自己的规则了吗？

嗯...不完全是。 如果你可以提前进行检查以保证此功能不会失败，那么我们就没有违反任何 runtime 开发规则。

让我们再看看 `transfer_from()` 中可能失败的检查：

* kitty 不存在所有者
* "from" 账户无法拥有相关的 kitty
* 从用户的 `owned_kitty_count` 中减去 kitty 时可能会出现下溢
* 将 kitty 添加到用户的 `owned_kitty_count` 时可能会出现溢出

因此，在我们可以按照我们想要的方式链接函数之前，我们需要确保所有这些检查都能成功：

1. 确保你的 `buy_kitty` 函数检查 kitty 有所有者
2. 使用该所有者的值直接驱动你的 `transfer_from()` 函数
3. 要确保如果存在 kitty 的所有者，那么该所有者的 kitty 计数大于 0（否则你在 runtime 中会遇到更大的问题）。
4. 确保 `all_kitties_count` 和 `owned_kitties_count` 使用相同的类型（`u64`）来跟踪小猫的数量。 我们的 mint 函数要确保 `all_kitties_count` 永远不会溢出。因为 `owned_kitties_count` 必须小于或等于 `all_kitties_count`，所以我们可以自信地说 `owned_kitties_count` 永远不会溢出。

因此，在我们的 `buy_kitty()` 函数的上下文中，我们实际上可以使用 `expect()` 来证明为什么这个函数不会失败。像这样：

``` rust
Self::transfer_from(owner.clone(), sender.clone(), kitty_id)
    .expect("`owner` is shown to own the kitty; \
    `owner` must have greater than 0 kitties, so transfer cannot cause underflow; \
    `all_kitty_count` shares the same type as `owned_kitty_count` \
    and minting ensure there won't ever be more than `max()` kitties, \
    which means transfer cannot cause an overflow; \
    qed");
```

你实际上将会在 [substrate 仓库](https://github.com/paritytech/substrate/search?q=expect) 中找到类似的地方。

请记住，作为区块链开发人员，你有责任验证其中的代码和逻辑的正确性。和智能合约平台不同，Substrate 框架不能在出现错误的情况下保护你。

花一点时间坐下想想上面所做的，因为当你开始用 substrate 开发自己的项目时要记住这一点很重要。

## 轮到你了！

按照提供的模板编程，在必要的代码中完成 `buy_kitty()` 函数。在 Polkadot-JS Apps UI 中测试你的新函数是否存在任何错误。

<!-- tabs:start -->

#### ** Template **

[embedded-code](../../3/assets/3.3-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](../../3/assets/3.3-finished-code.rs ':include :type=code embed-final')

<!-- tabs:end -->
