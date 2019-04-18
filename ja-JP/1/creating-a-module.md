モジュールを作成する
===

はじめに、ランタイム用に新しいモジュールを作成する必要があります。そのために、空のモジュールテンプレートを使って作業します。これを新しい`substratekitties.rs`ファイルに配置します。

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

このテンプレートを使うことによりモジュールの最も基本的な部分である、パブリック関数とストレージを簡単に書きはじめることができます。

しかし、それを始める前に同ディレクトリにある `lib.rs`ファイルに定義されている全体のランタイムに、このランタイムファイルを含める必要があります。

## ランタイムのアップデート

`lib.rs`ファイルをよく見ると、ランタイムを構成するすべてのモジュールについての詳細が含まれていることがわかります。モジュールごとに以下のことをする必要があります：

 - モジュールを含むRustファイルをインポートする
 - その「`トレイト`」を実装する
 - モジュールを `construct_runtime！`マクロに含める

よって、私たちのランタイムファイルにも同様のことをします。

作成した新しいモジュールファイルを含めるためには、私たちのファイルの先頭近く(`// Add this line`コメントで示されている)に次の行を追加します：

```rust
// `lib.rs`
...
pub type BlockNumber = u64;

pub type Nonce = u64;

// Add this line
mod substratekitties;
...
```

まだモジュールには何も定義していないので、`トレイト`の実装は非常にシンプルです。これは他のトレイト実装の下に書きます。

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

最後にこのラインを`construct_runtime!`の最後に追加しましょう。

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
この定義に3つの `types`である`Module`、`Call`、`Storage`を追加したことに注目してください。これらは全て、テンプレートで定義されたマクロによって生成されています。

正しく追加できていれば、このコードは問題なくコンパイルするでしょう。試して見ましょう！

```bash
./build.sh
cargo build --release
```

## 演習してみよう!

まだ実際に手を動かしていない場合は、このページの指示に従ってあなたの`substrate-node-template`を作成してください。全部間違いなく書けたなら、エラーなしでコードを正常にコンパイルできるはずです：

```bash
./build.sh
cargo build --release
```

このチュートリアルの各セクションの最後には、コードがエラーなくコンパイルできるように設計されています。このチュートリアルでの作業の大部分は `substratekitties.rs`ファイルで行われますが、後々もう一度`lib.rs`ファイルを更新する必要があります。

それでは、楽しいハッキングを開始しましょう！

<!-- tabs:start -->

#### ** 解答 **

[embedded-code](../../1/assets/1.1-finished-code.rs ':include :type=code embed-final')

<!-- tabs:end -->

---
**詳細解説**

`construct_runtime!`マクロのドキュメンテーションは[ここ](https://docs.substrate.dev/docs/construct_runtime)で確認してください。

---
