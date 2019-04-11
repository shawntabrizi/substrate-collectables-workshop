イベントを作成する
===

Substrateでは、[**Transactions**](https://docs.substrate.dev/docs/glossary#section-transaction)は、Ethereumで慣れ親しんでいたものとは異なる方法で処理されます。トランザクションが確定されたとしても、必ずしもそのトランザクションによって実行された機能が完全に成功したことを意味するわけではありません。

それを知るためには、関数の最後に[**`Event`**](https://docs.substrate.dev/docs/glossary#section-events)を発行して、成功を報告するだけでなく、特定の状態遷移が起こったことを「チェーン外の世界」に伝えます。

## イベントを宣言する

Substrateは `decl_event！`マクロを提供しており、これを使ってランタイムロジックに簡単にイベントを作成することができます。

以下はイベント宣言の例です：

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

繰り返しになりますが、カスタムのSubstrateタイプを使用したい場合は、[ジェネリクス](https://doc.rust-jp.rs/the-rust-programming-language-ja/1.6/book/generics.html)をイベント定義に統合する必要があります。

## イベントタイプを追加する

`decl_event！`マクロではあなたのモジュールで公開する新しい`Event`タイプを生成します。このタイプは以下のようにいくつかの特性を継承する必要があります：

```rust
pub trait Trait: balances::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}
```

##イベントをデポジットする

ランタイム内でイベントを使用するためには、それらのイベントをデポジットする関数を追加する必要があります。これはランタイム開発では一般的なパターンなので、[**`decl_module！`**](https://github.com/paritytech/wiki/pull/272)マクロは自動的にこのデフォルト実装をあなたのモジュールに追加することができます。

以下の様にあなたのモジュールに新しい関数を追加するだけです：

```rust
fn deposit_event<T>() = default;
```

あなたのイベントがジェネリクスを使用しない場合（例えばRustプリミティブ型のみ）、このように `<T>`を省略しましょう：

```rust
fn deposit_event() = default;
```

`decl_module！`マクロはこの行を検出し、それをあなたのランタイムに適した関数定義と置き換えます。

### `deposit_event()`を呼び出す

モジュール内にイベント関数を設定できたので、実際にイベントをデポジットしてみましょう。

それをするには、関数の最後にあなたの`Event`定義に設定した値を入力する必要があります：

```rust
let my_value = 1337;
let my_balance = <T::Balance as As<u64>>::sa(1337);

Self::deposit_event(RawEvent::MyEvent(my_value, my_balance));
```

## `lib.rs`を更新してイベントを導入する

ランタイムをコンパイルさせる最後のステップは、あなたが定義した新しい`Event`タイプを含むように`lib.rs`ファイルを更新することです。

モジュールの`Trait`実装には、以下の様に導入する必要があります：

```rust
// `lib.rs`
...
impl mymodule::Trait for Runtime {
    type Event = Event;
}
...
```

最後に、`construct_runtime！`マクロの中でモジュールの定義に`Event`あるいは`Event<T>`型も含める必要があります。どちらを含めるかは、イベントでジェネリクスを使用しているかどうかによって異なります。

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

## あなたの番です!

まずは、イベントをサポートするためにあなたのモジュールを更新する必要があります。ここまでで必要なことは全て教えましたので、あとはあなたがそれを全部つなぎ合わせなければなりません。

このセクションのテンプレートの手順を使用して、モジュールのイベント導入を進めましょう。`lib.rs`を更新する必要があることも忘れないようにしてください。

<!-- tabs:start -->

#### ** Template **

[embedded-code-template](../../2/assets/2.2-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](../../2/assets/2.2-finished-code.rs ':include :type=code embed-final')

<!-- tabs:end -->
