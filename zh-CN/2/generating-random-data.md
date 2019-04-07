# 生成随机数据

在最后一节的结尾，我们允许每个用户创建自己的 kitty。但是，它们并不是独一无二的... 让我们解决这个问题。

## 生成一个随机数种子

如果我们想要区分这些 kitties，我们需要给它们独一无二的属性！对于我们的应用程序而言，我们需要为每个 kitty 和一些随机 `dna` 生成一个唯一的 `id`。

我们可以使用 `system` module 从我们的链中安全地获取一些随机性：

```rust
<system::Module<T>>::random_seed()
```

Substrate 使用一个安全混合的算法，该算法使用先前块的熵来为每个后续块生成新的随机数据。

但是，由于它依赖于先前的块，因此可能需要超过 80 个块才能完全预热，并且你可能注意到了种子在此之前不会改变。

### 使用 Nonce

由于随机种子不会因同一个块中的多个交易而发生变化，并且它可能不会为前 80 个块生成随机种子，所以我们还引入了一个我们可以管理的 `nonce`。此外，我们还可以使用像 `AccountId` 这样的用户特定属性来引入更多的熵。

如果我们能够 hash 这些组合数据，我们应该就可以为我们的需求提供足够的随机性。

## 哈希数据

Substrate 上的随机数生成器看起来像这样：

```rust
let sender = ensure_signed(origin)?;
let nonce = <Nonce<T>>::get();
let random_seed = <system::Module<T>>::random_seed();

let random_hash = (random_seed, sender, nonce).using_encoded(<T as system::Trait>::Hashing::hash);

<Nonce<T>>::mutate(|n| *n += 1);
```

`Nonce` 将成为我们存储中的新的一项，当我们使用它的时候，我们只需要简单地增加它就行了。

我们可以使用 `random_hash` 来填充我们的 kitty 的 `id` 和 `dna`。

## 检查碰撞

标准化我们的逻辑以使用唯一 id 作为我们存储项的全局密钥，对轻松追踪我们的所有 kitties是很有帮助的。这意味着单个唯一键将指向我们的 `Kitty` 对象，而且所有其他所有权链接或映射将会指向该键。

`Kitty` 对象上的 `id` 将用于此目的，但我们需要确保新 kitty 的 `id` 始终是唯一的。我们可以使用新的存储项目 `Kitties` 来实现这一点，它将是从 `id` (`Hash`) 到 `Kitty` 对象的映射。

使用此对象，我们可以通过简单地检查此存储项是否已包含了特定 `id` 的映射来轻松检查冲突。就像这样：

```rust
ensure!(!<Kitties<T>>::exists(new_id), "This new id already exists");
```

由于两个随机生成的哈希值可能会发生碰撞，因此，当我们可能引入其他生成 kitties 的方法，进行此检查非常重要，我们的存储结构取决于这种唯一性。

## 轮到你了！

让我们更新我们的 module 来为我们的 kitty 生成随机数据提供支持，并使用该随机数据创建唯一的 `id`，这将成为我们存储和逻辑的基础。

我们将介绍一个新的 `Kitties` 存储项，它将 `id` 映射到 `Kitty` 对象。我们还想创建一个 `KittyOwner` 存储项，它将 `id` 映射到拥有 kitty 的用户的 `AccountId`。

最后，我们可以更新 `OwnedKitty` 对象以指向此唯一 `id`，而不是在我们的存储中存储 `Kitty` 对象的副本。

现在我们已经设置了所有这些新的存储项，我们还需要更新我们的 `create_kitty()` 函数，以便在制作 kitty 时正确更新这些存储项。

按照本节模板上的说明操作！

<!-- tabs:start -->

#### ** Template **

[embedded-code](../../2/assets/2.1-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](../../2/assets/2.1-finished-code.rs ':include :type=code embed-final')

<!-- tabs:end -->
