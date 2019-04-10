Creating an Event
===

イベントを作成する
===

Substrateでは、[** Transactions **]（https://docs.substrate.dev/docs/glossary#section-transaction）は、Ethereumで慣れ親しんでいたものとは異なる方法で処理されます。トランザクションが確定されたとしても、必ずしもそのトランザクションによって実行された機能が完全に成功したことを意味するわけではありません。

それを知るために、関数の最後に[** `Event` **]（https://docs.substrate.dev/docs/glossary#section-events）を発行して、成功を報告するだけでなく、特定の状態遷移が起こったことを「チェーン外の世界」に伝えなさい。

On Substrate, [**Transactions**](https://docs.substrate.dev/docs/glossary#section-transaction) are handled differently than you might have been used to on Ethereum. Even though a transaction may be finalized, it does not necessarily imply that the function executed by that transaction fully succeeded.

To know that, we should emit an [**`Event`**](https://docs.substrate.dev/docs/glossary#section-events) at the end of the function to not only report success, but to tell the "off-chain world" that some particular state transition has happened.

## Declaring an Event

##イベントを宣言する

Substrateは `decl_event！`マクロを提供しており、これを使ってランタイムロジックに入れることができるイベントを簡単に作成することができます。

これはイベント宣言の例です。

Substrate provides a `decl_event!` macro which allows you to easily create events which you can deposit in your runtime logic.

Here is an example of an event declaration:

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

Again, if we want to use some custom Substrate types, we need to integrate generics into our event definition.

繰り返しますが、カスタムのSubstrateタイプを使用したい場合は、総称をイベント定義に統合する必要があります。

##イベントタイプを追加する

`decl_event！`マクロはあなたのモジュールで公開する必要がある新しい `Event`タイプを生成します。このタイプは以下のようにいくつかの特性を継承する必要があります。

## Adding an Event Type

The `decl_event!` macro will generate a new `Event` type which you will need to expose in your module. This type will need to inherit some traits like so:

```rust
pub trait Trait: balances::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}
```

## Depositing an Event

##イベントをデポジットする

ランタイム内でイベントを使用するためには、それらのイベントを蓄積する関数を追加する必要があります。これはランタイム開発では一般的なパターンなので、[** `decl_module！` **]（https://github.com/paritytech/wiki/pull/272）マクロは自動的にこれのデフォルト実装をあなたのモジュールに追加することができます。

単にあなたのモジュールに新しい関数を追加するだけです。

In order to use events within your runtime, you need to add a function which deposits those events. Since this is a common pattern in runtime development, the [**`decl_module!`**](https://github.com/paritytech/wiki/pull/272) macro can automatically add a default implementation of this to your module.

Simply add a new function to your module like so:

```rust
fn deposit_event<T>() = default;
```
あなたのイベントが総称を使用しない場合（例えばRustプリミティブ型のみ）、このように `<T>`を省略するべきです：

If your events do not use any generics (e.g. just Rust primitive types), you should omit the `<T>` like this:

```rust
fn deposit_event() = default;
```

`decl_module！`マクロはこの行を検出し、それをあなたのランタイムに適した関数定義と置き換えます。

The `decl_module!` macro will detect this line, and replace it with the appropriate function definition for your runtime.

### Calling `deposit_event()`

### deposit_event（）を呼び出す

モジュール内に設定されたものが揃ったので、関数の最後に実際にイベントをデポジットすることをお勧めします。

それをすることは比較的簡単です、あなたはあなたの `Event`定義と一緒に行く値を提供する必要があります：

Now that you have things set up within your module, you may want to actually deposit the event at the end of your function.

Doing that is relatively simple, you just need to provide the values that go along with your `Event` definition:

```rust
let my_value = 1337;
let my_balance = <T::Balance as As<u64>>::sa(1337);

Self::deposit_event(RawEvent::MyEvent(my_value, my_balance));
```

## Updating `lib.rs` to Include Events

ランタイムをコンパイルさせる最後のステップは、あなたが定義した新しい `Event`タイプを含むように` lib.rs`ファイルを更新することです。

あなたのモジュールの `Trait`実装には、含める必要があります：

The last step to get your runtime to compile is to update the `lib.rs` file to include the new `Event` type you have defined.

In your modules `Trait` implementation, you need to include:

```rust
// `lib.rs`
...
impl mymodule::Trait for Runtime {
    type Event = Event;
}
...
```

最後に、 `construct_runtime！`マクロの中でモジュールの定義に `Event`あるいは` Event <T> `型も含める必要があります。どちらを含めるかは、イベントで総称を使用しているかどうかによって異なります。

Finally, you need to also include the `Event` or `Event<T>` type to your module's definition in the `construct_runtime!` macro. Which one you include depends again on whether your event uses any generics.

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

## Your Turn!

イベントをサポートするためにあなたのモジュールを更新する必要があるでしょう。私たちはあなたにすべての異なる部分を教えました、あなたはそれを一緒につなぐ必要があるでしょう。

このセクションのテンプレートの手順を使用して、モジュールの話を進めましょう。忘れずに `lib.rs`も更新する必要があります。

You will need to update your module to support events. We have taught you all the different parts, you will just need to piece it together.

Use the instructions in the template of this section to help you get your module talking! Remember you will need to update `lib.rs` too!

<!-- tabs:start -->

#### ** Template **

[embedded-code-template](./assets/2.2-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](./assets/2.2-finished-code.rs ':include :type=code embed-final')

<!-- tabs:end -->
