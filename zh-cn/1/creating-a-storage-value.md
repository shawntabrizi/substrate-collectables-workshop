# 创建一个 Storage Value

让我们将最简单的逻辑添加到 runtime 中：一个存储变量的函数。

为此，我们首先需要在 [**`decl_storage!`**](https://substrate.dev/rustdocs/v1.0/srml_support_procedural/macro.decl_storage.html) 宏中为 [**Storage Item**](https://substrate.dev/docs/en/overview/glossary#storage-items) 定义存储变量。Substrate 存储数据库允许类型安全的用法，因此你可以在区块之间保持一致。

## 声明一个 Storage Value

Substrate 本身支持 Rust 中可用的所有原始类型（`bool`，`u8`，`u32` 等）以及一些 Substrate 中的特定自定义类型 (`AccountId`, `Balance`, `Hash`, [and more](https://polkadot.js.org/api/types/)...)

你可以声明一个简单的 storage item，如下所示：

```rust
decl_storage! {
    trait Store for Module<T: Trait> as Example {
        MyU32: u32;
        MyBool get(my_bool_getter): bool;
    }
}
```

这里我们定义了两个变量：一个 `u32` 变量和一个带有 getter 函数 `my_bool_getter` 的 `bool` 变量。`get` 参数是可选的，但如果将其添加到 storage item，它将公开具有指定名称的 getter 函数（`fn getter_name() -> Type`）。

要存储这些基本存储值，你需要导入 `support::StorageValue`  module。

### 使用 Storage Value 工作

用于访问 `StorageValue` 的函数被定义在 [`srml/support` 文件夹](https://github.com/paritytech/substrate/blob/master/srml/support/src/storage/generator.rs#L98) 中：

```rust
/// Get the storage key.
fn key() -> &'static [u8];

/// true if the value is defined in storage.
fn exists<S: Storage>(storage: &S) -> bool {
    storage.exists(Self::key())
}

/// Load the value from the provided storage instance.
fn get<S: Storage>(storage: &S) -> Self::Query;

/// Take a value from storage, removing it afterwards.
fn take<S: Storage>(storage: &S) -> Self::Query;

/// Store a value under this key into the provided storage instance.
fn put<S: Storage>(val: &T, storage: &S) {
    storage.put(Self::key(), val)
}

/// Mutate this value
fn mutate<R, F: FnOnce(&mut Self::Query) -> R, S: Storage>(f: F, storage: &S) -> R;

/// Clear the storage value.
fn kill<S: Storage>(storage: &S) {
    storage.kill(Self::key())
}
```

所以如果你想 "put" `MyU32` 的值，你可以这样写：

```rust
<MyU32<T>>::put(1337);
```

如果你想 "get" `MyBool` 的值，你可以选择以下任一一种写法：

```rust
let my_bool = <MyBool<T>>::get();
let also_my_bool = Self::my_bool_getter();
```

我们将在下一节中向你展示如何将这些调用集成到你自己的 module 中。

## 轮到你了！

创建一个名为 `Value` 的存储值，用于存储 `u64`。

确保编译器所需的任何库都被导入了。你的代码应该能编译成功。

<!-- tabs:start -->

#### ** Template **

[embedded-code](../../1/assets/1.2-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](../../1/assets/1.2-finished-code.rs ':include :type=code embed-final')

<!-- tabs:end -->
