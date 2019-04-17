# 追踪所有 Kitties

现在我们已经让每个用户都可以创建自己独一无二的 kitty，我们开始追踪它们！

我们的游戏将会追踪创建的 kitty 总数，以及追踪谁拥有哪只 kitty。

## "Verify First, Write Last"

> 重要提示：此部分非常重要

作为基于 Substrate 框架开发应用的开发人员，很重要的一点是要区分 Substrate 上 runtime 的逻辑设计和 Ethereum 平台上的智能合约开发。

在 Ethereum 中，如果你的交易在任何时候失败（错误，没有 gas 等...），你的智能合约的状态将不受影响。但是，在 Substrate 上并非如此。一旦交易开始修改区块链的存储，这些更改就是永久性的，即使交易在 runtime 执行期间失败也是如此。

这对于区块链系统是必要的，因为你可能想要追踪用户的 nonce 或者为任何发生的计算减去 gas 费用。对于失败的交易来说，这两件事实际上都发生在 Ethereum 状态转换函数中，但你作为智能合约开发人员，从来不必担心去管理这些事情。

既然现在你是 Substrate runtime 开发人员，你必须察觉到你对区块链状态所做的任何更改，并确保它遵循 **"verify first, write last"** 的模式。我们将在整个教程中帮助你做到这点。

## 创建一个 List

Substrate 本身不支持 list 类型，因为它可能会导致开发者养成危险的习惯。在 runtime 开发中，列表迭代通常是坏事。除非明确防范了其危险操作，否则它将为仅花费 O(1) 复杂度的操作添加无限制的 O(N) 复杂度。结果就是你的链变得容易被攻击。作为替代，你可以使用映射和计数器模拟列表，如下所示：

```rust
decl_storage! {
    trait Store for Module<T: Trait> as Example {
        AllPeopleArray get(person): map u32 => T::AccountId;
        AllPeopleCount get(num_of_people): u32;
    }
}
```

这里我们将在 runtime 中存储人员列表，用多个 `AccountId` 表示。我们只需要小心谨慎地维护这些存储项目，以确保它们准确和最新。

## 检查 Overflow/Underflow

如果你曾经在 Ethereum 上开发过，那么如果你不执行 “safe math”，你就会碰到你所熟悉的问题，即 Overflow/Underflow。Overflow 和 Underflow 很容易就可以使 runtime 出现 panic 或者存储混乱。

在更改存储状态之前，你必须始终主动检查可能的 runtime 错误。请记住，与 Ethereum 不同，当交易失败时，状态不会恢复到交易发生之前，因此你有责任确保在错误处理上不会产生任何副作用。

幸运的是，在 Rust 中检查这些类型的错误非常简单，其中原始数字类型具有 [`checked_add()`](https://doc.rust-lang.org/std/primitive.u32.html#method.checked_add) 和 [`checked_sub()`](https://doc.rust-lang.org/std/primitive.u32.html#method.checked_sub) 函数。

假设我们想要向 `AllPeopleArray` 中添加一项，我们首先要检查我们是否可以成功增加 `AllPeopleCount`，如下所示：

```rust
let all_people_count = Self::num_of_people();

let new_all_people_count = all_people_count.checked_add(1).ok_or("Overflow adding a new person")?;
```

使用 `ok_or` 与下面的代码相同：

```rust
let new_all_people_count = match all_people_count.checked_add(1) {
    Some (c) => c,
    None => return Err("Overflow adding a new person"),
};
```

但是，`ok_or` 比 `match` 更清晰可读; 你只需要确保记住在末尾加 `?`！

如果我们成功地能够在没有上溢的情况下递增 `AllPeopleCount`，那么它就会将新值分配给 `new_all_people_count`。如果失败，我们的 module 将返回一个 `Err()`，它可以由我们的 runtime 优雅地处理。错误消息也将直接显示在节点的控制台输出中。

## 更新存储中的 List

现在我们已经检查过了，我们可以安全地增加列表项，我们最终可以将更改推送到存储中。请记住，当你更新列表时，列表的 “最后一个索引” 比计数少一个。例如，在包含 2 个项的列表中，第一个项是索引 0，第二个项是索引 1。

将新的人员添加到我们的人员列表中，完整示例如下所示：

```rust
fn add_person(origin, new_person: T::AccountId) -> Result {
    let sender = ensure_signed(origin)?;

    let all_people_count = Self::num_of_friends();

    let new_all_people_count = all_people_count.checked_add(1).ok_or("Overflow adding a new person")?;

    <AllPeopleArray<T>>::insert(all_people_count, new_people);
    <AllPeopleCount<T>>::put(new_all_people_count);

    Ok(())
}
```

我们也应该为这个函数添加碰撞检测！你还记得怎么做吗？

## 删除 List 元素

当我们尝试从列表中间删除元素时，映射和计数模式引入的一个问题就是会在列表中留下空位。幸运的是，在本教程中我们管理的列表的顺序并不重要，因此我们可以使用 "swap and pop" 的方法来有效地缓解此问题。

"swap and pop" 方法交换删除项的位置以及列表中的最后一项。然后，我们可以简单地删除最后一项而不会在我们的列表中引入任何空位。

我们不会在每次删除时运行循环来查找删除项的索引，而是使用一些额外的存储来追踪列表中每个项及其所在的位置。

我们现在不会引入 "swap and pop" 的逻辑，但是我们会要求你使用一个 `Index` 存储项来追踪列表中每项的索引，如下所示：

```rust
AllPeopleIndex: map T::AccountId => u32;
```

这实际上只是 `AllPeopleArray` 中内容的反转。请注意，我们这里不需要 getter 函数，因为此存储项只在内部使用，并且不需要作为模块 API 的一部分公开。

## 轮到你了！

现在你已经知道了追踪所有 kitties 的必要性，让我们更新你的 runtime 来执行此操作。

按照模板添加新的存储项，更新你的 `create_kitty` 函数，每当创建新的 kitty 时都要 **"verify first, write last"**。

<!-- tabs:start -->

#### ** Template **

[embedded-code](../../2/assets/2.3-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](../../2/assets/2.3-finished-code.rs ':include :type=code embed-final')

<!-- tabs:end -->

---

**Learn More**

聊聊 Rust 中 clone 和 borrow...

[TODO: make this a page]

---
