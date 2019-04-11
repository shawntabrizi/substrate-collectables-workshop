構造体を保存する
===

もしみんな自分のユニークな番号を持てたなら？このクールなアイデアを全員にデジタルキティを配って実現しよう！

最初に、キティたちが持つ特性を`struct`の形で定義する必要があり、それからこれらのカスタム`struct`をランタイムストレージに格納する方法を学びます。

## カスタム構造体を定義する

以下のようにあなたのランタイム用にカスタム構造体を定義することができます：

```rust
#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct MyStruct<A, B> {
    some_number: u32,
    some_generic: A,
    some_other_generic: B,
}
```

他の言語で構造体を定義するのと比べて、これはかなり普通に見えるはずです。しかし、ランタイム開発に関するこの宣言については、2つの奇妙な点があります。

カスタムの`Encode`と`Decode`特性を使うには、それらを `parity_codec`クレートからインポートする必要があります：

```rust
use parity_codec::{Encode, Decode};
```

### ジェネリックタイプを使う

私たちが格納している型の1つとしてジェネリックを使って私たちの例の構造体を定義していることに気付くでしょう。構造体の中で `AccountId`や`Balance`のようなカスタムのSubstrate型を使おうとすると、構造体を使うたびにこれらの型を渡す必要があるので、これは重要になります。

`some_generic`に`Balance`を、そして`some_other_generic`に`Hash`を格納したいのであれば、以下のようにストレージアイテムを定義する必要があります:

```rust
decl_storage! {
    trait Store for Module<T: Trait> as Example {
        MyItem: map T::AccountId => MyStruct<T::Balance, T::Hash>;
    }
}
```

For the purposes of clarity, we will name a generic type for `T::AccountId` as `AccountId` or `T::Balance` as `Balance`. You can use comma separate and add more generics as needed following this pattern.
わかりやすくするために、`T::AccountId`タイプを`AccountId`、そして`T::Balance`タイプを`Balance`と名付けます。このパターンに従い、必要に応じてカンマ区切りを使用してジェネリックを追加できます。

### 派生マクロ

あなたが気づくもう一つのことは一番上の `＃[derive（...）]`です。これはRustコンパイラによって提供される属性で、いくつかの特性の基本的な実装を可能にします。 2行目、`＃[cfg_attr(feature ="std", derive(Debug))]`は `Debug`トレイトについても同じことを行いますが、"標準 "ライブラリを使うとき、すなわちネイティブバイナリをコンパイルするときだけです。Wasmでは行いません。詳しく学びたい方は[ここ](https://doc.rust-lang.org/rust-by-example/trait/derive.html)を参照してください。このチュートリアルでは、おまじないだと思ってください。

## モジュール関数内のカスタム構造体

ランタイム記憶域でカスタム構造体を初期化したので、次に値をプッシュして変更することができます。

モジュール関数を使用して構造体を作成して記憶域に挿入する例をお見せします：

```rust
decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn create_struct(origin, value: u32, user: T::AccountId) -> Result {
            let sender = ensure_signed(origin)?;

            let new_struct = MyStruct {
                some_number: value,
                some_generic: user,
            };

            <MyItem<T>>::insert(sender, new_struct);
            Ok(())
        }
    }
}
```

## Your Turn!

u64の代わりに`Kitty`構造体を格納するようにストレージマッピングランタイムを更新してください。

`Kitty`には以下のプロパティを持たせましょう：

 - `id` : `Hash`
 - `dna` : `Hash`
 - `price` : `Balance`
 - `gen` : `u64`

あなたのために`create_kitty（）`関数のスケルトンが作成されていますので、その中にロジックを追加してください。`Kitty`オブジェクトを使って`new_kitty`を作成し、そのオブジェクトをあなたのランタイムストレージに保存するコードを含めてください。

あなたの`T::Hash`と`T::Balance`を初期化するには以下を実行してください：

```rust
let hash_of_zero = <T as system::Trait>::Hashing::hash_of(&0);

let my_zero_balance = <T::Balance as As<u64>>::sa(0);
```

`gen`は`0`から始めましょう。

<!-- tabs:start -->

#### ** Template **

[embedded-code](../../1/assets/1.6-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](../../1/assets/1.6-finished-code.rs ':include :type=code embed-final')

<!-- tabs:end -->

---
**詳細解説**

### SubstrateでのString

あなたはもしかしたらキティに加える特性の一つが名前(String)であるはずだと思うかもしれません！実際、愛するものに名前がないことほど悲しいことはないでしょう。

残念ながらSubstrateは`Strings`を直接サポートしません。ランタイムストレージは、ランタイムが動作するビジネスロジックの状態を格納するためにあります。UIが必要とする一般的なデータを格納するのではありません。ランタイムに任意のデータを本当に保存する必要がある場合は、常にバイト配列（`Vec <u8>`）を作成することができますが、もっと合理的な方法はIPFSのようなサービスにハッシュを保存してからフェッチすることです。これは現在このワークショップの範囲外ですが、キティに関する追加のメタデータをサポートする目的で今後のアップデートで追加されるかもしれません。

---
