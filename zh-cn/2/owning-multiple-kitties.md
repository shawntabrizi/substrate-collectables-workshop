# 拥有多个 Kitties

你是否注意到我们的 runtime 存在问题？如果同一个用户调用 `create_kitty()` 函数会发生什么？

现在我们的存储只能追踪每个用户的一个 kitty，但我们的 kitty 总数将会继续增加！让我们通过让用户拥有多个 kitty 来解决这个问题！

## 使用 Tuples 去模拟高阶数组

我们需要引入一些更复杂的存储项来表示多个用户对多个项目的所有权。

幸运的是，根据我们的需求，使用一个由 `AccountId` 和 `Index` 组成的 [tuple](https://doc.rust-lang.org/rust-by-example/primitives/tuples.html) 几乎就可以解决我们的问题了。

以下是我们如何使用这样的结构构造每个人独有的 "friends list"：

```rust
MyFriendsArray get(my_friends_array): map (T::AccountId, u32) => T::AccountId;
MyFriendsCount get(my_friends_count): map T::AccountId => u32;
```

这模拟了一个标准的二维数组，如：

```rust
MyFriendsArray[AccountId][Index] -> AccountId
```

我们也可以获取用户的朋友数量：

```rust
MyFriendsArray[AccountId].length()
```

## 相对索引

与以前一样，我们可以通过索引项目的位置来优化 runtime 需要执行的计算工作。一般的方法是反转 `MyFriendsArray` 的映射，并创建一个这样的存储项：

```rust
MyFriendsIndex: map (T::AccountId, T::AccountId) => u32;
```

如果 `AccountId` 代表用户和他们的朋友, 那么返回值将是 `MyFriendsArray` 的索引，即该朋友在该用户的朋友列表中的存储位置。

但是，由于我们的 kitty 都具有唯一标识符作为 `Hash`，并且不能被多个用户所拥有，所以我们实际上可以简化此结构

```rust
MyKittiesIndex: map T::Hash => u32;
```

这个索引告诉了我们对于一个给定的 kitty，可以在哪里查看该项的 *所有者* 数组。

## 轮到你了！

这里应该没有什么太令人惊讶的了... 只是做了一点会计的工作。

按照模板去引入我们最终的一系列存储项，并确保 `create_kitty()` 函数正确管理这些存储项。它应该与上一节非常相似。

<!-- tabs:start -->

#### ** Template **

[embedded-code](../../2/assets/2.4-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](../../2/assets/2.4-finished-code.rs ':include :type=code embed-final')

<!-- tabs:end -->

---

**Learn More**

聊聊 Option<T::AccountID>

[TODO: make this a page]

---
