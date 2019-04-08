# Storage Mapping

我们之前的 runtime 只允许为区块链中的所有用户存储单个值。让我们更多地思考下我们的链，显然为每个用户存储其各自的值也是有用的。

为此，我们将使用 storage mapping 替换简单的 storage value。

## Substrate 特定类型

在我们进入 storage mapping 之前，让我们来谈谈我们将要使用的一些 substrate 特定类型。

你的默认 runtime 模板包含一组 modules，这些 modules 暴露出了你期望从一条区块链中获得的类型。在你开发了更多 module 后，你甚至可能会发现自己会暴露出新类型给 runtime 的其他部分。

在本教程中，我们将仅使用 3 种 substrate 特定类型：

1. AccountId
2. Balance
3. Hash

我们的 module 本身不能访问这些类型，但我们可以通过让 module 的 `Trait` 继承定义了这些类型的 module 来轻松获取访问权限。在这种情况下，`balances` module 有我们需要的一切东西：

```rust
pub trait Trait: balances::Trait {}
```

像我们在上面的示例中所做的那样，我们可以在指定了泛型 `<T: Trait>` 的任何地方通过使用 `T::Type` 来访问这些类型。

## 声明一个 Storage Map

Storage Map 允许你将基本 (key, value) 对放入 runtime 存储中。它可以像这样声明：

```rust
decl_storage! {
    trait Store for Module<T: Trait> as Example {
        SomeValue get(some_value_getter): map u32 => u32;
        MyValue: map T::AccountId => u32;
    }
}
```

你可以看到，当你想要表示 “owned” 数据时，mapping 是非常有用的。由于我们可以创建从某个用户（AccountId）到某些值（例如MyValue）的 mapping，因此我们可以保留有关该用户的存储信息。我们甚至可以在 runtime 中构建逻辑，使 runtime 可以管理具体哪些用户能被允许修改那些值，这是我们将在本教程中使用的常见模式。

要使用 storage map，你需要导入 `support::StorageMap` 类型。

### 使用 StorageMap 工作

用于访问 `StorageMap` 的函数与 `StorageValue` 的位于[同一位置](https://github.com/paritytech/substrate/blob/master/srml/support/src/storage/generator.rs#L162)：

```rust
/// Get the prefix key in storage.
fn prefix() -> &'static [u8];

/// Get the storage key used to fetch a value corresponding to a specific key.
fn key_for(x: &K) -> Vec<u8>;

/// true if the value is defined in storage.
fn exists<S: Storage>(key: &K, storage: &S) -> bool {
    storage.exists(&Self::key_for(key)[..])
}

/// Load the value associated with the given key from the map.
fn get<S: Storage>(key: &K, storage: &S) -> Self::Query;

/// Take the value under a key.
fn take<S: Storage>(key: &K, storage: &S) -> Self::Query;

/// Store a value to be associated with the given key from the map.
fn insert<S: Storage>(key: &K, val: &V, storage: &S) {
    storage.put(&Self::key_for(key)[..], val);
}

/// Remove the value under a key.
fn remove<S: Storage>(key: &K, storage: &S) {
    storage.kill(&Self::key_for(key)[..]);
}

/// Mutate the value under a key.
fn mutate<R, F: FnOnce(&mut Self::Query) -> R, S: Storage>(key: &K, f: F, storage: &S) -> R;
```

因此，如果要将值 “insert” 到一个 Storage Map 中，则需要提供 key 和 value，如下所示：

```rust
<SomeValue<T>>::insert(key, value);
```

你可以使用以下任一方法查询该值：

```rust
let my_value = <SomeValue<T>>::get(key);
let also_my_value = Self::some_value_getter(key);
```

## 轮到你了！

更新你的简单存储示例，现在存储一个从 `AccountId` 映射到 `u64` 的 map 吧。

<!-- tabs:start -->

#### ** Template **

[embedded-code](../../1/assets/1.4-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](../../1/assets/1.4-finished-code.rs ':include :type=code embed-final')

<!-- tabs:end -->

---

**Learn More**

谈谈 funding accounts 如何去发起交易

[TODO: make this a page]
