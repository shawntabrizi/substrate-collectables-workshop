# 介绍

在本节中，我们将向你展示创建自定义 runtime 的基础知识：

- 如何使用 runtime 存储
- 如何公开 runtime 函数
- 如何使用 Polkadot-JS Apps UI 与你的 runtime 进行交互

## 什么是 Runtime？

简而言之，[*Runtime*](https://docs.substrate.dev/docs/glossary#section-runtime) 是区块链的块执行逻辑，有时称为状态转换函数 [**STF**](https://docs.substrate.dev/docs/glossary#section-stf-state-transition-function-)。在 [Substrate](https://docs.substrate.dev/docs/glossary#section-substrate) 中，它以 WebAssembly 二进制文件这种实现中立，机器可执行的格式存储在链上 。其他系统倾向于仅以人类可读的格式来表达（例如 Ethereum）或者就根本不存在（例如 Bitcoin）。

## 什么是 Module？

你的区块链 runtime 由多个特性和功能组成，这些特性和功能共同为区块链提供支持。像：

- 帐户管理
- Token 余额
- 治理
- Runtime 升级
- 和更多...

[这些](https://github.com/paritytech/substrate/tree/master/srml)是代码库中提供的所有 modules，你可以轻松地将其包含在 runtime 中。Substrate 提供的这些默认 module 集合被称为 Substrate Runtime Module Library [**SRML**](https://docs.substrate.dev/docs/glossary#section-srml-substrate-runtime-module-library-)

使用 Substrate 框架，你可以轻松地在 runtime 中创建新的 module。这就是我们在本教程中将要做的！

## Rust

目前，Substrate 和 Runtime 开发使用 [Rust编程语言](https://www.parity.io/why-rust/)。

本教程 **不是** 学习 Rust 的课程，但我们应该回顾一下在使用本指南时与用其他语言编程时相比可能遇到的一些基本差异。

### Ownership 和 Borrowing

引用自 [Rust docs](https://doc.rust-lang.org/book/ownership.html):

> Ownership is Rust’s most unique feature, and it enables Rust to make memory safety guarantees without needing a garbage collector.
>
> - Each value in Rust has a variable that’s called its owner.
> - There can only be one owner at a time.
> - When the owner goes out of scope, the value will be dropped.

你将在整个教程中看到我们将在一些变量前面添加一个与号（＆），这意味着我们正在借用该值。如果我们需要在整个函数中多次重用该值，这种方式将非常有用。

它基本上阻止了你在处理内存时会犯的一些愚蠢错误，所以当 Rust 编译器建议你不要做某件事，请心存感激。

### Traits

引用自 [Rust docs](https://doc.rust-lang.org/book/traits.html):

> Traits abstract over behavior that types can have in common.

如果你熟悉 interfaces，Traits 是 [Rust 中唯一的 interface 概念](https://blog.rust-lang.org/2015/05/11/traits.html)。[**译者注**： traits 不能完全说成是 interface，它是属于 Rust 独有的部分。不过对常用 OOP 范式的开发者来看，它在使用上的确更接近 interface 的概念，但对常用 FP 范式的开发者来说，它在使用上更接近于 Haskell 中的 typeclass 概念]

### 使用 Result 创建 Recoverable Errors

稍后你将了解到模块函数必须返回 `Result` 类型，这允许我们处理函数中的错误。返回结果是 `Ok()` 时表示成功，是 `Err()` 时表示失败。

在本教程中，我们将在返回 `Result` 的函数末尾使用问号运算符(`?`)。当调用这样的函数时，例如 `my_function()?`, Rust 编译器简单地将代码扩展为:

```rust
match my_function() {
    Ok(value) => value,
    Err(msg) => return Err(msg),
}
```

你可以在[此处](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)了解更多相关信息。

下面是开发过程中会碰到的典型错误

```
error[E0382]: borrow of moved value: `s`
error[E0382]: use of moved value: `s`
error[E0502]: cannot borrow `s` as immutable because it is also borrowed as mutable
error[E0499]: cannot borrow `s` as mutable more than once at a time
```

例如，以下代码将无法编译：

```rust
fn main() {
    let mut s = String::from("hello");
    let t1 = &s;      // t1 is an immutable reference to the String
    let t2 = &mut s;  // t2 is a mutable reference to the String
    t2.push_str(&t1); // We want to append t2 to t1.
                      //
              // This is broken since both are references to the same underlying string.
}
```

Borrowing error:

```
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> src/main.rs:4:14
  |
3 |     let t1 = &s;
  |              -- immutable borrow occurs here
4 |     let t2 = &mut s;
  |              ^^^^^^ mutable borrow occurs here
5 |     t2.push_str(&t1);
  |                 --- immutable borrow later used here
```

简单修复这种错误的方法是克隆字符串而不是对它进行引用。

```rust
fn main() {
    let mut s = String::from("hello");
    let t1 = s.clone();
    let t2 = &mut s;
    t2.push_str(&t1);
}
```

Works!

### Macros

引用自 [Rust docs](https://doc.rust-lang.org/book/macros.html):

> While functions and types abstract over code, macros abstract at a syntactic level.

更简单地说，宏是编写代码的代码，通常用于简化代码或使代码更具可读性。

Substrate 在整个 Runtime 开发过程中使用了很多宏，它们支持十分特有的语法，但是它们返回的错误可读性相当差。[**译者注**： 推荐使用 [cargo-expand 工具](https://github.com/dtolnay/cargo-expand) 来展开宏，以便理解具体调用逻辑]

---

**Learn More**

Result 和 ? 如何工作？

介绍 Traits 和 interfaces 的关系？

[TODO: make this a page]

---
