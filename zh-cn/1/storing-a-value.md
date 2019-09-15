# 存储一个 Value

现在我们已经在 runtime 中声明了存储值，我们实际上可以创建一个函数来将值推送到存储中。

## 声明一个 Public Function

我们需要去定义设置和修改存储值的 runtime 函数。这可以在我们的 `decl_module!` 宏中完成，该宏声明了 module 用于处理逻辑的所有入口。

以下是声明公开函数的示例：

```rust
// Add these imports: 
//
// use support::{dispatch::Result, StorageValue};
// use system::ensure_signed;
decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        fn my_function(origin, input_bool: bool) -> Result {
            let _sender = ensure_signed(origin)?;

            <MyBool<T>>::put(input_bool);
            
            Ok(())
        }
    }
}
```

## 函数结构

此处公开的 module 函数应始终采用以下形式：

```rust
fn foo(origin, bar: Bar, baz: Baz, ...) -> Result;
```

### Origin

这些函数的第一个参数始终是 `origin`。 `origin` 包含有关调用来源的信息。通常分为三组：

- 由外部帐户签名的 public 调用。
- 允许仅由治理系统进行的 root 调用。
- 允许仅由块作者和验证者进行的 inherent 调用。

请参阅 Substrate 术语表中的 [Origin](https://substrate.dev/docs/en/overview/glossary#origin) 定义。

### Result

此外，这些函数必须返回 `support::dispatch` 模块中的 `Result` 类型。这意味着成功的函数调用将始终返回 `Ok(())`，否则应捕获可能导致问题的任何错误并返回 `Err()`。

由于这些是调度函数，因此需要记住两件非常重要的事情：

- MUST NOT PANIC: 在任何情况下（保存，或者存储进入一个不可挽回的损坏状态）函数都不能 panic。
- NO SIDE-EFFECTS ON ERROR: 此函数要么全部完成并返回 `Ok(())`，要么它必须对存储没有副作用并返回 `Err('Some reason'）`。

我们稍后会谈到这些。在本教程中，我们将确保满足这两个条件，并且我们也提醒你这样做。

## 检查签名消息

如上所述，任何这些模块函数中的第一个参数是 `origin`。`system` module 中提供了三个方便的可调用函数 `ensure_signed`, `ensure_root` 和 `ensure_inherent`，它们可以帮你做相应的匹配，并返回一个可用的结果。**在你的函数中做的第一件事应该总是从这三个调用函数中选择匹配的函数**。

我们可以使用 `system` 中的 `ensure_signed()` 函数来检查 origin，并“确保”消息是经过有效帐户签名的。我们甚至可以从函数调用的结果中派生签名帐户，如上面的示例函数所示。

## 轮到你了！

使用该模板创建一个 `set_value()` 函数，该函数允许用户发送一条签名消息，该消息将 `u64` 放入 runtime 存储中。

<!-- tabs:start -->

#### ** Template **

[embedded-code](../../1/assets/1.3-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](../../1/assets/1.3-finished-code.rs ':include :type=code embed-final')

<!-- tabs:end -->

---

**Learn More**

如果你尝试编译我们上面显示的代码示例而不导入所需的库，你将收到一些错误：

```rust
error[E0425]: cannot find function `ensure_signed` in this scope
  --> src/substratekitties.rs:15:27
   |
15 |             let _sender = ensure_signed(origin)?;
   |                           ^^^^^^^^^^^^^ not found in this scope
help: possible candidate is found in another module, you can import it into scope
   |
2  | use system::ensure_signed;
   |
```

如你所见，我们在代码中添加了一些尚未导入 module 的功能。实际上可以通过 Rust 编译器返回的错误消息里的建议来帮助我们解决问题。如果我们听从 Rust，那么我们可以简单地在最顶层添加这些 `use` 语句来让我们的代码再次编译通过：

```rust
use system::ensure_signed;
```

正如 “[common patterns](../0/common-patterns-moving-forward.md)” 部分所述，在整个 runtime 开发过程中 Rust 编译器将成为你的朋友，并且可以帮助你解决代码中的 *大部分* 问题。之后，当你需要导入一个新库时，我们都会尽力提起，但在编译器向你发送一些错误时不要担心，接受它可能给你的有用建议。

---
