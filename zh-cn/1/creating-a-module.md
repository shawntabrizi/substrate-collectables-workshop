# 创建一个 Module

首先，我们需要为 runtime 创建一个新 module。为此，我们将使用一个空 module 模板，并把它放在一个新的 `substratekities.rs` 文件中：

```
substratekitties
|
+-- runtime
    |
    +-- src
        |
        +-- lib.rs
        |
        +-- * substratekitties.rs
```

**substratekitties<span>.</span>rs**

```rust
use support::{decl_storage, decl_module};

pub trait Trait: system::Trait {}

decl_storage! {
    trait Store for Module<T: Trait> as KittyStorage {
        // Declare storage and getter functions here
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Declare public functions here
    }
}
```

你可以看到，该模板允许我们开始编写 module 的最基本部分：公共函数和存储。

但在我们开始这样做之前，我们应该将此文件包含在我们的整个 runtime 中，runtime 被定义在同一目录中的 `lib.rs` 文件中。

## 更新我们的 Runtime

如果仔细查看 `lib.rs` 文件，你会注意到它包含有关构成 runtime 的所有 module 细节。对于每个 module，我们：

- 导入包含该 module 的 Rust 文件
- 实现其 `Trait`
- 将该 module 包含在 `construct_runtime!` 宏中

所以我们需要在这里做同样的事情。

要包含我们创建的新 module 文件，我们可以在文件顶部附近添加下面这行（用注释 `// Add this line` 指出）:

```rust
// `lib.rs`
...
pub type BlockNumber = u64;

pub type Nonce = u64;

// Add this line
mod substratekitties;
...
```

由于我们没有在 module 中定义任何内容，因此我们的 `Trait` 实现也非常简单。我们可以在其他 trait 实现之后包含这一行：

```rust
// `lib.rs`
...
impl sudo::Trait for Runtime {
    type Event = Event;
    type Proposal = Call;
}

// Add this line
impl substratekitties::Trait for Runtime {}
...
```

最后，我们可以在 `construct_runtime!` 的末尾添加一行定义：

```rust
// `lib.rs`
...
construct_runtime!(
    pub enum Runtime with Log(InternalLog: DigestItem<Hash, Ed25519AuthorityId>) where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        System: system::{default, Log(ChangesTrieRoot)},
        Timestamp: timestamp::{Module, Call, Storage, Config<T>, Inherent},
        Consensus: consensus::{Module, Call, Storage, Config<T>, Log(AuthoritiesChange), Inherent},
        Aura: aura::{Module},
        Indices: indices,
        Balances: balances,
        Sudo: sudo,
        // Add this line
        Substratekitties: substratekitties::{Module, Call, Storage},
    }
);
...
```

请注意，我们在此定义中添加了三种类型（`Module`，`Call`，`Storage`），所有这些类型都是由模板中定义的宏生成的。

这样，这段代码是有效的，并且应该能编译通过。试一试：

```bash
./scripts/build.sh
cargo build --release
```

## 轮到你了！

如果你还没有准备好，请按照此页面上的说明设置你的 `substrate-node-template`。如果你已成功完成所有操作，则应该能够成功编译代码而不会出现错误：

```bash
./scripts/build.sh
cargo build --release
```

在本教程的每个部分的末尾，你的代码在编译时应该没有错误。本教程中的大多数更改都将在 `substratekities.rs` 文件中进行，但我们需要稍后再次更新 `lib.rs` 文件。

现在是时候开始添加一些我们自己的逻辑了！

<!-- tabs:start -->

#### ** Solution **

[embedded-code](../../1/assets/1.1-finished-code.rs ':include :type=code embed-final')

<!-- tabs:end -->

---

**Learn More**

查看 `construct_runtime!` 宏的[文档](https://substrate.dev/rustdocs/v1.0/srml_support/macro.construct_runtime.html)。

---
