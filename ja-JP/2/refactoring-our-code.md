コードのリファクタリング
===

ここまでのコードを見ると`create_kitty()`関数はかなり汚くなってしまいました。後で交配や他の方法でキティを「ミント」する方法を導入することを考え、少しリファクタリングが必要でしょう。

この機会を利用して、ランタイムのAPIから呼び出しができない、プライベートな内部関数の作り方を教えます。

## パブリックインタフェースとプライベート関数

ランタイム内には、モジュールの実装を以下のように行うことができます。

```rust
impl<T: Trait> Module<T> {
    // Your functions here
}
```

このブロック内の関数は通常、パブリックインタフェースまたはプライベート関数です。パブリックインタフェースは`pub`とラベル付けされ、一般的にストレージに書き込まないインスペクタ関数とそうするオペレーション関数に分類されます。プライベート関数は他のモジュールに利用できない実装されたランタイム内部でしか呼び出しができないプライベートユーティリティです。

ここで定義した関数は、以前扱った`Self::function_name()`パターンを使って呼び出すことができます。以下の例は意図的に複雑にしています：

```rust
decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn adder_to_storage(origin, num1: u32, num2: u32) -> Result {
            let _sender = ensure_signed(origin)?;
            let result = Self::_adder(num1, num2);

            Self::_store_value(result)?;

            Ok(())
        }
    }
}

impl<T: Trait> Module<T> {
    fn _adder(num1: u32, num2: u32) -> u32 {
        let final_answer = num1.checked_add(num2).ok_or("Overflow when adding")?;
    }

    fn _store_value(value: u32) -> Result {
        <myStorage<T>>::put(value);
        
        Ok(())
    }
}
```

ここでも「最初に検証し、最後に書き込み」というパターンに従う必要があることを忘れないでください。したがって、エラーが発生する可能性がある場合にストレージに書き込みを行うプライベート関数をデイジーチェーン接続(数珠つなぎ)しないことが重要です。

## あなたの番です!

私たちにとって、`Kitty`オブジェクトから新しいキティを作成し、すべてのストレージ変数を更新するプロセスは、同じコードを利用しながら他のキティを作成できるように再利用可能にすべきものです。

`create_kitty()`関数は生成されたキティが常にトランザクションを送信したユーザに渡されると仮定していますが、私たちの再利用可能な `mint()`関数はその限りではありません。

`create_kitty()`関数は新しいキティのプロパティも生成しますが、これを `mint()`関数に移すべきではありません。将来、キティオブジェクトを生成する方法は複数あるかもしれません。この機能は、作成されているキティがユニークであること、およびすべてのストレージインデックスが正しく更新されていることを確認するためだけに使用します。

`mint()`の関数宣言のためのテンプレートは既に提供されています。それに従い、コードをリファクタリングしてください。先に進む前に、もう一度Polkadot-JS Apps UIで作業内容を確認することをお勧めします。

<!-- tabs:start -->

#### ** Template **

[embedded-code](../../2/assets/2.6-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](../../2/assets/2.6-finished-code.rs ':include :type=code embed-final')

<!-- tabs:end -->
