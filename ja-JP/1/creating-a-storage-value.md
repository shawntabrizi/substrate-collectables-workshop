ストレージの値を作成する
===
実行可能な最も簡単なロジックであるストレージ（変数を格納する関数）を追加しましょう。

これを行うには、最初に[**Storage Item**](https://vdocs.substrate.dev/docs/glossary#section-storage-items)のストレージ変数を[**`decl_storage！`**](https://crates.parity.io/srml_support_procedural/macro.decl_storage.html)マクロで定義する必要があります。これにより、Substrateストレージデータベースのタイプセーフな使用が可能になるため、ブロック間の状況を把握することができます。

## ストレージ変数と宣言する

SubstrateはRustで利用可能なすべてのプリミティブ型(`bool`、`u8`、`u32`など）と、Substrate固有のカスタム型（`AccountId`、`Balance`、`Hash`、[その他](https://polkadot.js.org/api/types/)...)をネイティブでサポートしています。

以下の様に書くことで簡単なストレージアイテムを宣言することができます：

```rust
decl_storage! {
    trait Store for Module<T: Trait> as Example {
        MyU32: u32;
        MyBool get(my_bool_getter): bool;
    }
}
```

ここでは2つの変数を定義しました：`my_bool_getter`という名前の取得関数を持つ`u32`と `bool`です。 `get`パラメータはオプションですが、ストレージアイテムに追加することにより、指定された名前を持つgetter関数を使えるようになります(`fn getter_name() -> Type`)。

これらの基本的なストレージ値を保存するためには、`support::StorageValue`というモジュールをインポートする必要があります。

### ストレージ値の作業を行う

`StorageValue`にアクセスするために使われる関数は[`srml/support`フォルダ](https://github.com/paritytech/substrate/blob/master/srml/support/src/storage/generator.rs#L98)に定義されています。：

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

`MyU32`に値を"put"したいなら、以下のように書けます：

```rust
<MyU32<T>>::put(1337);
```

`MyBool`の値を"get"したいなら、以下のどちらかの方法で書けます：

```rust
let my_bool = <MyBool<T>>::get();
let also_my_bool = Self::my_bool_getter();
```

次のセクションでは、これらの呼び出しを自分のモジュールに統合する方法を説明します。

## 演習してみよう!

`u64`を格納する`Value`という格納値を作成してください。

コンパイラーが必要とする必要なライブラリーを必ずインポートしてください。するとコードは正常にコンパイルされるはずです。

<!-- tabs:start -->

#### ** テンプレート **

[embedded-code](../../1/assets/1.2-template.rs ':include :type=code embed-template')

#### ** 解答 **

[embedded-code-final](../../1/assets/1.2-finished-code.rs ':include :type=code embed-final')

<!-- tabs:end -->
