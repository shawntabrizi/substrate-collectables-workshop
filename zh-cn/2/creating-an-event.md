# 创建一个 Event

在Substrate上，[**Transactions**](https://docs.substrate.dev/docs/glossary#section-transaction) 的处理方式与以往在 Ethereum 上的处理方式不同。即使 transaction 可能已经完成，但并不一定意味着该 transaction 执行的函数完全成功。

为了知道执行函数是否完全成功，我们应该在函数结束时发出一个 [**`Event`**](https://docs.substrate.dev/docs/glossary#section-events)，不仅要报告成功，还要告诉 "off-chain world" 某些特定的状态转换已经发生了。

## 声明一个 Event

Substrate 提供了一个 `decl_event!` 宏，它允许你轻松创建可以存储在 runtime 逻辑中的 event。

以下是声明 event 的示例：

```rust
decl_event!(
    pub enum Event<T>
    where
        <T as system::Trait>::AccountId,
        <T as system::Trait>::Balance
    {
        MyEvent(u32, Balance),
        MyOtherEvent(Balance, AccountId),
    }
);
```

同样，如果我们想要使用一些自定义 Substrate 类型，我们需要将泛型集成到我们的 event 定义中。

## 添加一个 Event 类型

`decl_event!` 宏将生成一个新的 `Event` 类型，你需要在 module 中暴露出它。这种类型需要继承一些 traits，如下所示：

```rust
pub trait Trait: balances::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}
```

## Depositing 一个 Event

为了在 runtime 中使用 event，你需要添加一个 deposit 这些 events 的函数。由于这是 runtime 开发中的常见模式，因此 [**`decl_module!`**](https://github.com/paritytech/wiki/pull/272) 宏可以在你的 module 中自动添加一个默认实现。

只需在 module 中添加一个新函数，如下所示：

```rust
fn deposit_event<T>() = default;
```

如果你的 event 不使用任何泛型（例如只是 Rust 原始类型），你应该省略 `<T>`，如下所示：

```rust
fn deposit_event() = default;
```

`decl_module!` 宏将会检测出此行，在你的 runtime 中用合适的函数定义来替换它。

### 调用 `deposit_event()`

现在你已经在 module 中构建了一些东西，你可能想要在函数结束时 deposit event。

这样做相对简单，你只需提供与 `Event` 定义中类型相同的值：

```rust
let my_value = 1337;
let my_balance = <T::Balance as As<u64>>::sa(1337);

Self::deposit_event(RawEvent::MyEvent(my_value, my_balance));
```

## 更新 `lib.rs` 来包含 Events

让 runtime 编译成功的最后一步是更新 `lib.rs` 文件以包含你定义的新 `Event` 类型。

在你的 module 的 `Trait` 实现中，你需要包括：

```rust
// `lib.rs`
...
impl mymodule::Trait for Runtime {
    type Event = Event;
}
...
```

最后，你还需要在 `construct_runtime!` 宏中包含 `Event` 或`Event<T>` 类型到你的 module 定义中。你包含哪一个取决于你的 event 是否使用了泛型。

```rust
// `lib.rs`
...
construct_runtime!(
    pub enum Runtime with Log(InternalLog: DigestItem<Hash, Ed25519AuthorityId>) where
        Block = Block,
        NodeBlock = opaque::Block,
        InherentData = BasicInherentData
    {
        ...
        MyModule: mymodule::{Module, Call, Storage, Event<T>},
    }
);
...
```

## 轮到你了！

你需要更新 module 以支持 event。我们已经教了你所需的所有部分，你只需要把它拼凑起来。

使用本节模板中的说明来帮助你的 module 运行起来! 记住你也需要更新 `lib.rs`！

<!-- tabs:start -->

#### ** Template **

[embedded-code-template](../../2/assets/2.2-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](../../2/assets/2.2-finished-code.rs ':include :type=code embed-final')

<!-- tabs:end -->
