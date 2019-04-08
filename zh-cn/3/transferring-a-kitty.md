# 转让 Kitty

现在我们已经允许用户拥有多个 kitties 了，我们可能还想添加一个功能，允许一个用户将他们拥有的 kitty 转让给另一个用户。

所有权完全由我们的存储管理，因此 `transfer_kitty` 函数实际上只是修改现有存储以反映新状态。

让我们考虑一下我们需要更新的存储项目：

- 更改全局 kitty 所有者
- 更改每个用户拥有的 kitty 数量
- 更改 kitty 的拥有 kitty 索引
- 更改每个用户拥有的 kitty 映射

## Swap and Pop

我们在本教程前面提到过，我们将使用 “swap and pop” 方法从我们的临时列表中删除项目。重要的是要记住，这种方法会导致我们改变列表的顺序，但是非常有效。

如果要构建 runtime 时，列表顺序对业务逻辑是有影响的，则需要重新考虑如何管理它，但这不是算法课程，因此我们将为你提供 runtime 的工作代码。

```rust
let kitty_index = <OwnedKittiesIndex<T>>::get(kitty_id);

if kitty_index != new_owned_kitty_count_from {
    let last_kitty_id = <OwnedKittiesArray<T>>::get((from.clone(), new_owned_kitty_count_from));
    <OwnedKittiesArray<T>>::insert((from.clone(), kitty_index), last_kitty_id);
    <OwnedKittiesIndex<T>>::insert(last_kitty_id, kitty_index);
}
```

请注意，我们在代码中已经包含了一个小优化，如果我们要删除的索引不是数组中的最后一个元素，它只需要进行一次 “swap”。在那种情况下，只需要一次 “pop”。

## 轮到你了！

要进行转让，你只需要使用到目前为止所有学到的工具，不需要任何什么新东西。

我们希望使 transfer 函数可重用，因此我们已经构建了模板来帮助你。

按照模板完成 `transfer_from()` 私有函数，为你 module 中暴露出的公有 `transfer()` 函数提供支持。

<!-- tabs:start -->

#### ** Template **

[embedded-code](../../3/assets/3.2-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](../../3/assets/3.2-finished-code.rs ':include :type=code embed-final')

<!-- tabs:end -->
