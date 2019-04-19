ストレージマッピング
===

前回のランタイムでは、ブロックチェーンの単一の値を保存することしかできませんでした。私達が目指しているコレクタブルチェーンの開発を考えたとき、各ユーザーのために別の形でストレージに保存できる必要があるでしょう。

これを可能にするために、単純な単一値ストレージをストレージマッピングに置き換えます。

## Substrate特有のタイプ

ストレージマッピングに入る前に、使用するSubstrate特有のタイプについて説明します。

デフォルトのランタイムテンプレートには、ブロックチェーンから取得可能なタイプを公開する多数のモジュールが含まれています。モジュールの開発を進めるにつれて、ランタイムの他の部分に新しいタイプを公開することさえあるかもしれません。

このチュートリアルでは、以下の3種類のSubstrate特有のタイプのみを使用します：

 1. AccountId
 2. Balance
 3. Hash

私たちのモジュールは本来これらの型にアクセスすることはできませんが、モジュールの`Trait`にこれらの型を定義したモジュールから継承させることでアクセスが可能になります。この場合、私達に必要なものは全て`balances`モジュールに備わっています：

```rust
pub trait Trait: balances::Trait {}
```

上の例ではTraitという名前のtraitにbalances::Traitを継承しています。
`T::Type`を使用することにより、ジェネリックな`<T：Trait>`を指定したところならどこでもこれらの型にアクセスできます。

## ストレージマップの宣言

ストレージマップを使用すると、基本的な（key, value）のペアをランタイムストレージに格納putできます。これは次のように宣言できます。

```rust
decl_storage! {
    trait Store for Module<T: Trait> as Example {
        SomeValue get(some_value_getter): map u32 => u32;
        MyValue: map T::AccountId => u32;
    }
}
```

「所有されている」データを表したい場合、マッピングは非常に便利な格納方法となります。ユーザ（`AccountId`）から`MyValue`のような値へのマッピングを作成することができるので、そのユーザに関する情報をkey毎に保存することができます。このチュートリアルでは誰がそれらの値を変更できるかを管理するロジックをランタイムで構築する方法も学びます。

ストレージマップを使うには、`support::StorageMap`タイプをインポートする必要があります。

### ストレージマップを使って作業する

`StorageMap`にアクセスするために使われる関数は[StorageValueと同じ場所](https://github.com/paritytech/substrate/blob/master/srml/support/src/storage/generator.rs#L162)にあります：

```rust
/// Get the prefix key in storage.
fn prefix() -> &'static [u8];

/// Get the storage key used to fetch a value corresponding to a specific key.
fn key_for(x: &K) -> Vec<u8>;

/// true if the value is defined in storage.
fn exists<S: Storage>(key: &K, storage: &S) -> bool {
    storage.exists(&Self::key_for(key)[..])
}

/// Load the value associated with the given key from the map.
fn get<S: Storage>(key: &K, storage: &S) -> Self::Query;

/// Take the value under a key.
fn take<S: Storage>(key: &K, storage: &S) -> Self::Query;

/// Store a value to be associated with the given key from the map.
fn insert<S: Storage>(key: &K, val: &V, storage: &S) {
    storage.put(&Self::key_for(key)[..], val);
}

/// Remove the value under a key.
fn remove<S: Storage>(key: &K, storage: &S) {
    storage.kill(&Self::key_for(key)[..]);
}

/// Mutate the value under a key.
fn mutate<R, F: FnOnce(&mut Self::Query) -> R, S: Storage>(key: &K, f: F, storage: &S) -> R;
```

そのため、StorageMappingに値を「挿入」したい場合は、次のようにキーと値を指定する必要があります：

```rust
<SomeValue<T>>::insert(key, value);
```

そしてその値を以下のいずれかの方法でクエリ(問い合わせ)することができます：

```rust
let my_value = <SomeValue<T>>::get(key);
let also_my_value = Self::some_value_getter(key);
```

## 演習してみよう!

簡単なストレージ例題を更新して、今度は `AccountId`から`u64`へのマップを保存します。

<!-- tabs:start -->

#### ** テンプレート **

[embedded-code](../../1/assets/1.4-template.rs ':include :type=code embed-template')

#### ** 解答 **

[embedded-code-final](../../1/assets/1.4-finished-code.rs ':include :type=code embed-final')

<!-- tabs:end -->

---
**発展**

トランザクションを行うためにアカウントに預金する

[TODO: make this a page]

---
