イントロダクション
===

このセクションでは、カスタムランタイム作成のベーシックを教えます：

- ランタイムストレージの使い方
- ランタイム機能のエクスポーズ方法
- Polkadot-JS Apps UIを使ったランタイムへのアクセス方法

## ランタイムとは?

簡単に言うと、[*Runtime*](https://substrate.dev/docs/en/overview/glossary#runtime)はブロックチェーンのブロック実行ロジックで、時にステイトトランジションファンクション([**STF**](https://substrate.dev/docs/en/overview/glossary#stf-state-transition-function))とも呼ばれます。[Substrate](https://substrate.dev/docs/en/overview/glossary#substrate)では, これは、実装に依存しない、機械実行可能な形式でWebAssemblyバイナリとしてチェーン上に保存されます。他のシステムはそれを人間が読める形式（例えばEthereum）で表現するか、まったく表現しない（例えばBitcoin）などがあります。

## モジュールとは?

ブロックチェーンのランタイムは、あなたのブロックチェーンを構成する複数の機能です。例えば：

- アカウント管理
- トークン残高
- ガバナンス
- ランタイムアップグレード
- など...

これら全てのモジュールはランタイムに簡単に導入できるように[コードベース](
https://github.com/paritytech/substrate/tree/master/srml)で提供されています。Substrateが提供するこれらのデフォルトのモジュールのセットはSubstrate Runtime Module Library ([**SRML**](https://substrate.dev/docs/en/overview/glossary#srml-substrate-runtime-module-library))と呼ばれます。

Substrateフレームワークを使用すると、新しいモジュールを簡単に作成してランタイムに導入することができます。それがこのチュートリアルでやることです！

## Rust

現時点では、SubstrateとRuntimeの開発は[Rustプログラミング言語](https://www.parity.io/why-rust/)を使用しています。

このチュートリアルは、**Rustを学ぶコースではありません** が、他の言語でのプログラミングと比較して、このガイドに従うときに遭遇する可能性のある基本的な違いのいくつかを紹介します。

### 所有権と借用

[Rust docs](https://doc.rust-lang.org/book/ownership.html)からの引用：

>所有権はRustの最もユニークな機能であり、これによりRustはガベージコレクタを必要とせずにメモリの安全性を保証できます。
>
>  - Rustのそれぞれの値は、所有者と呼ばれる変数を持ちます。
>  - 一度に所有者は一人だけしか存在できません。
>  - 所有者がスコープから外れるとになると、値は破棄されます。

チュートリアルを通して、いくつかの変数の前にアンパサンド（ `＆`）を追加します。これは、値を借用していることを意味します。これは、関数全体で値を複数回再利用する必要がある場合に便利です。

これによりメモリ処理でのくだらないミスを防ぐことができるので、エラーを教えてくれるRustコンパイラー様には感謝をしましょう。

### トレイト

[Rust docs](https://doc.rust-lang.org/book/traits.html)からの引用:
> トレイトはタイプが持つ共通の振る舞いを定義します。

トレイトは[Rustにおいてインターフェイスを記述するただ一つの概念](https://blog.rust-lang.org/2015/05/11/traits.html)です。

### Resultで回復可能なエラー

モジュール関数は、関数内のエラーを処理するために、`Result`型を返さなければなりません。`Result`は、成功した場合は` Ok() `、失敗した場合は` Err()`を返します。

このチュートリアルを通して、`Result`を返す関数の最後に疑問符演算子（`?`）を使います。このような関数、例えば `my_function()?`を呼び出すとき、Rustは単にコードを以下のように展開します:

```rust
match my_function() {
    Ok(value) => value,
    Err(msg) => return Err(msg),
}
```

詳細を学びたい方は[ここ](https://doc.rust-jp.rs/book/second-edition/ch09-02-recoverable-errors-with-result.html)を参考にしてください。

典型的なエラー；

```
error[E0382]: borrow of moved value: `s`
error[E0382]: use of moved value: `s`
error[E0502]: cannot borrow `s` as immutable because it is also borrowed as mutable
error[E0499]: cannot borrow `s` as mutable more than once at a time
```

例えば、下のコードをコンパイルするとエラーを返します:

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

借用エラー：

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

このようなエラーに対する簡単な対処法は、stringを参照するのではなく、クローンすることです。

```rust
fn main() {
    let mut s = String::from("hello");
    let t1 = s.clone();
    let t2 = &mut s;
    t2.push_str(&t1);
}
```

やったね！

### マクロ

[Rust docs](https://doc.rust-jp.rs/book/second-edition/appendix-04-macros.html)からの引用:

> 基本的に、マクロは、他のコードを記述するコードを書く術であり、これはメタプログラミングとして知られています。メタプログラミングは、書いて管理しなければならないコード量を減らすのに有用で、これは、関数の役目の一つでもあります。

簡単に言えば、マクロはコードを書くコードであり、通常はコードを簡易化したり、読みやすくするのに使われます。

Substrateはランタイム開発プロセスで多くのマクロを使っており、シンタックス特化していたり、返されるエラー情報が乏しいことが結構あります。

---
**発展**

 Resultがどう動くか？

 トレイトの導入　→　インターフェイスとのコネクション？

[TODO: これに関するページを作る]

---
