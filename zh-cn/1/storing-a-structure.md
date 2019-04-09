# 存储一个 Structure

如果你认为每个人都有自己的 number 是一件很酷的事，那让我们试着给所有的数字 kitties 添加吧

首先，我们需要以 `struct` 的形式定义我们的 kitties 具有哪些属性，然后我们需要学习如何在 runtime 存储中存储这些自定义结构。

## 定义一个自定义 Struct

你可以为 runtime 定义一个自定义结构，如下所示：

```rust
#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct MyStruct<A, B> {
    some_number: u32,
    some_generic: A,
    some_other_generic: B,
}
```

与在其他语言中定义结构相比，这应该看起来很正常。但是，你会在 runtime 开发中注意到这个声明的两个奇怪之处。

要使用自定义的 `Encode` 和 `Decode` traits，你需要从 `parity_codec_derive` crate 中导入它们:

```rust
use parity_codec_derive::{Encode, Decode};
```

### 使用 Generics

你会注意到我们使用泛型定义了我们的示例结构来作为我们的一个存储类型。当我们尝试在结构中使用 `Substrate` 中的类型（如 `AccountId` 或 `Balances`）时，这显得非常重要，因为每次我们在使用我们的结构体时都需要传递这些类型。

因此，如果我们想在 `some_generic` 中存储 `AccountId`，我们需要像这样定义我们的存储项：

```rust
decl_storage! {
    trait Store for Module<T: Trait> as Example {
        MyItem: map T::AccountId => MyStruct<T::Balance, T::Hash>;
    }
}
```

为了清楚起见，我们将 `T::AccountId` 的泛型类型命名为 `AccountId`，将 `T::Balance` 的泛型类型命名为 `Balance`。你可以使用逗号分隔，并根据需要在此模式后添加更多泛型。

### Derive Macro

你会注意到的另一件事是顶部的 `＃[derive(...)]`。这是 Rust 编译器提供的属性，允许基本地实现某些 trait。第二行，`＃[cfg_attr(feature = "std", derive(Debug))]` 对 `Debug` trait 做了同样的事情，但仅在使用“标准”库时启用，即在编译本机二进制文件而不是 Wasm 的时候。你可以在[这里](https://doc.rust-lang.org/rust-by-example/trait/derive.html)了解更多相关信息。出于本教程的目的，你可以将其视为 magic。

## Module 函数中的自定义 Struct

现在我们已经在 runtime 存储中初始化了自定义结构，接着我们可以存储具体值并对其进行修改。

以下是使用 module 函数创建结构并将其插入到存储的示例：

```rust
decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn create_struct(origin, value: u32, balance: T::Balance, hash: T::Hash) -> Result {
            let sender = ensure_signed(origin)?;

            let new_struct = MyStruct {
                some_number: value,
                some_generic: balance,
                some_other_generic: hash,
            };

            <MyItem<T>>::insert(sender, new_struct);
            Ok(())
        }
    }
}
```

## 轮到你了！

更新你 runtime 中的 storage map 以存储 `Kitty` 结构体而不是 u64。

一个 `Kitty` 应该具有以下属性：

- `id` : `Hash`
- `dna` : `Hash`
- `price` : `Balance`
- `gen` : `u64`

我们已经为你创建了 `create_kitty()` 函数的框架，但你需要添加逻辑。具体逻辑包含使用 `Kitty` object 创建 `new_kitty` ，并将该 object 存储到 runtime 存储中。

要初始化 `T::Hash` 和 `T::Balance`，你可以使用：

```rust
let hash_of_zero = <T as system::Trait>::Hashing::hash_of(&0);

let my_zero_balance = <T::Balance as As<u64>>::sa(0);
```

我们也需要将 `gen` 初始化 `0`。

<!-- tabs:start -->

#### ** Template **

[embedded-code](../../1/assets/1.6-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](../../1/assets/1.6-finished-code.rs ':include :type=code embed-final')

<!-- tabs:end -->

---

**Learn More**

### Substrate 中的 Strings

你可能希望我们为 kitties 添加一个名称属性！毕竟，谁不会给他们喜欢的东西取名呢?

Substrate 不直接支持字符串。Runtime 存储用于存储 runtime 运行的业务逻辑的状态。它不是存储 UI 所需的一般数据。如果你确实需要将一些任意数据存储到 runtime，你总是可以创建一个 bytearray（`Vec<u8>`），但更合乎逻辑的做法是将哈希值存储到 IPFS 之类的服务中，然后获取数据用于 UI 展示。这超出了本次研讨会的范围，但可能会在以后为你的 kitty 添加其他元数据。

[**译者注**： 从编程语言角度来说，Substrate 之所以不直接支持 `String` 是因为 runtime 运行在 Wasm 上，而 Rust 编译为 Wasm 时需要启用 `no_std` 配置，也就是无法使用 `libstd` 标准库，这也就导致了标准库中的 `String` 无法在 runtime 逻辑中使用。但实际上可以直接用 `Vec<u8>` 来代替 `String`，因为 `String` 本身的存储就是一个遵循 `UTF8` 格式的 `Vec<u8>`，而 `Vec<u8>` 可以在不需要导入标准库的情况下通过导入 `liballoc` 库来使用，注意该库还只是 `nightly-only`]

---
