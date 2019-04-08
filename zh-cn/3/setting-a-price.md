# 设置 Kitty 的价格

我们链的核心已经完成！用户现在可以创建和追踪他们的 Substratekitties 的所有权。

但如果我们想让我们的 runtime 更像游戏，我们需要引入其他功能，如买卖。我们将首先让用户更新其 kitty 的价格来启动这些功能。

## 更新一个存储结构

每个 `Kitty` 对象都有一个我们默认设置为 0 的 `price` 属性。如果我们想要更新 kitty 的价格，我们需要下拉 `Kitty` 对象，更新价格，并将其推回存储中。

请记住，如果要更新值，Rust 希望你将变量声明为 mutable (`mut`)，因此我们应该在此处执行此操作：

```rust
let mut object = Self::get_object(object_id);
object.value = new_value;

<Objects<T>>::insert(object_id, object);
```

## 权限函数

任何用户都可以使用签名消息调用我们的 `create_kitty()` 函数，但是当我们创建修改对象的函数时，我们应该确认只有适当的用户才能成功进行这些函数调用。

要修改 `Kitty` 对象，我们需要获取 kitty 的 `owner`，并 `ensure` 它与 `sender` 相同。

`KittyOwner` 存储了一个映射到 `Option<T::AccountId>`，因为给定的 `Hash` 可能不会指向一个生成和拥有的 `Kitty`。这意味着，每当我们获取 kitty 的所有者时，我们需要解决它返回 `None` 的可能性。这可能是由于用户输入错误甚至是 runtime 的某些问题造成的，但检查有助于防止出现这类问题

我们 module 的所有权检查将如下所示：

```rust
let owner = Self::owner_of(object_id).ok_or("No owner for this object")?;

ensure!(owner == sender, "You are not the owner");
```

## 合理检查

我们开始让用户调用我们在 runtime 中暴露出的公共函数，这意味着我们的用户有机会无意或者甚至是恶意地提供不良输入。

我们需要确保我们的 runtime 始终进行健全性检查，使得用户不能破坏我们的链。如果我们要创建一个更新对象的值的函数，我们做的第一件事最好就是确保对象存在。

```rust
ensure!(<MyObject<T>>::exists(object_id));
```

## 轮到你了！

现在拥有更新 `Kitty` 对象所需的所有工具了。请记住，**"verify first, write last"**。确保在实际更改存储之前执行所有相应的检查，并且不要假设用户为你提供了良好的数据！

按照本节中的模板来帮助你创建 `set_price()` 函数。

<!-- tabs:start -->

#### ** Template **

[embedded-code](../../3/assets/3.1-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](../../3/assets/3.1-finished-code.rs ':include :type=code embed-final')

<!-- tabs:end -->
