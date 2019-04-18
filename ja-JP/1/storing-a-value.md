値を保存する
===

ランタイムにストレージ値を宣言したので、実際に値をプッシュする関数を作成できます。

## パブリック関数を宣言する

ストレージ値を設定および変更するランタイム関数を定義する必要があります。これは`decl_module！`マクロの中で行うことができ、このマクロはモジュールが扱うすべてのエントリポイントを宣言します。

これはパブリック関数宣言の一例です。

```rust
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

## ファンクション構造

パブリックなモジュール関数は以下のフォーマットに従う必要があります：

```rust
fn foo(origin, bar: Bar, baz: Baz, ...) -> Result;
```
### Origin

これらの関数の最初の引数は常に `origin`です。`origin`は呼び出しがどこから来たのかについての情報を含んでいます。これは通常、以下の3つのグループに分けられます。

 - 外部アカウントによって署名されているパブリックコール
 - ガバナンスシステムによってのみ許可されているルートコール
 - ブロックの作成者とバリデータによってのみ許可されている内部コール

詳しくはSubstrate Glossaryの[Originの定義](https://docs.substrate.dev/docs/glossary#section-origin)を参照してください。

### Result
さらに、これらの関数は `support::dispatch`モジュールから`Result`型を返さなければなりません。つまり、成功した関数コールは常に`Ok(())`を返し、エラーをキャッチした場合は`Err()`を返さなければなりません。

これらはディスパッチされた関数なので、2つの非常に重要なルールがあります。

 -  パニックにならないこと：いかなる状況下でも(おそらく、修復不可能な損傷を受けた状態になっているストレージをセーブして)この関数はパニックになってはいけません。
 - エラーが副作用を起こさないこと：完全に完了してOk(())を返さなければならないか、あるいはストレージに副作用を与えずにErr（その理由）を返さなければなりません。

これらの詳細は後に説明しますが、このチュートリアルを通して、上の二つの条件が満たされているかを確認してください。

## 署名されたメッセージを確認する

前述したように、これらのモジュール関数のいずれにおいても最初の引数は `origin`になります。`system`には、マッチングを行い便利な結果を返す3つの便利なコールがあります：`confirm_signed`、`ensure_root`、そして`confirm_inherent`です。 **関数を定義する際には、まず最初にこのいずれかのコールを使う必要があります**

`system :: confirm_signed`からの`confirm_signed（）`関数を使ってOriginをチェックし、メッセージが有効なアカウントで署名されていることを"確かめる"ことができます。上記の関数例のように、関数呼び出しの結果から署名アカウントを変数`sender`などに導出することもできます。

## 演習してみよう!

テンプレートを使用して `set_value（）`関数を作成します。これによりユーザーは署名されたメッセージを送信してランタイムストレージに`u64`を置くことができます。

<!-- tabs:start -->

#### ** Template **

[embedded-code](../../1/assets/1.3-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](../../1/assets/1.3-finished-code.rs ':include :type=code embed-final')

<!-- tabs:end -->

---
**詳細解説**

必要なライブラリをインポートせずに上記のコードサンプルをコンパイルしようとすると、エラーが発生します。

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

ご覧のとおり、まだモジュールにインポートしていない機能がコードに追加されています。Rustのエラーメッセージはこの様な問題を解決するのに非常に役に立ちます。Rustが言うには、コンパイルを成功させるためには`use`ステートメントを一番上に追加する必要があります：

```rust
use system::ensure_signed;
```

「進行上の一般的なパターン」のセクションで述べたように、Rustはランタイム開発全体を通してあなたの問題を解決する良き友となるでしょう。問題があるときにRustコンパイラが発するエラーは決して怒っているのではありません。あなたがよりセキュアな、美しいコードを書けるようにお手伝いしているのです。受け入れてください。

"どんな結果に対しても、僕はそれを受け入れる。
失敗したときの自分の立場が怖いからといって、変な理由づけはしません。
だから僕の発している言葉に嘘はないはずです。"

イチロー

---
