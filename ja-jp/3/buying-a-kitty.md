キティを買う
===

キティの価格を設定し、キティの所有権を転送できるようになったので、 `buy_kitty`関数を構築するために必要なものはすべて揃いました。

## キティが売りに出されていることを確認する

ユーザーが`buy_kitty()`関数を実行することを許可する前に、キティが本当に売りに出されていることを確かめるべきです。キティの`price`がデフォルト値の`0`の場合は、売りに出されていないとし、例を単純化しました。所有者は自分のキティの値段を`set_price()`を呼び出して0よりも大きい値にすることで、簡単に市場に出すことができます。

タイプに公開されている関数を使うことで、`T::Balance`がゼロであることを簡単に確認することができます:

```rust
let my_value = <T::Balance as As<u64>>::sa(0);
ensure!(my_value.is_zero(), "Value is not zero");
// `ensure` will succeed and execution continues here
```

これに改善を加えるとすれば、価格を`Option<T::Balance>`にすることにより、0を有効な価格とすることです。0の代わりに、売りに出されていないものは`None`で表せます。これは読者へのチャレンジとして置いておきます。

## 支払いをする

これまで、私たちのチェーンは`Balances`モジュールによって提供されるチェーン内部通貨から完全に独立していました。`Balances`モジュールを使うことによって、全ユーザーの通貨を完全に管理することができます。しかしそれは、使い方に非常に注意を払う必要があることを意味します。

幸いなことに、`Balances`モジュールは`Currency`と呼ばれるトレイトを公開しており、それは[`transfer()`](https://substrate.dev/rustdocs/v1.0/srml_support/traits/trait.Currency.html#tymethod.transfer)と呼ばれる関数を実装しています。この`transfer()`関数は、通常なら必要である多くの検証(オーバーフロー、アンダーフロー、残高確認など)を行ってくれ、安全かつ簡単にアカウント間の送金を可能にします。

`Currency`トレイトと、それにより実装された機能にアクセスするには、以下のようにトレイトをあなたのモジュールにインポートする必要があります：

```rust
use support::traits::Currency;
```

その後、以下のようにトレイト内の特定の機能にアクセスできます：

```
<balances::Module<T> as Currency<_>>::transfer(&from, &to, value)?;
```

## 再注意："まず検証せよ、書き込むのは最後だ"

transfer()関数の実装を見ると、「検証」と「書き込み」の両方が行われます。つまり、モジュールのロジックの一部として含める場所には注意が必要です:

```rust
// end of verifications

<balances::Module<T> as Currency<_>>::transfer(&from, &to, value)?;

// beginning of writing to storage
```

テンプレートを見ての通り、私たちは`transfer()`を呼び出した直後に `transfer_from()`関数を実行することをお勧めします。

しかしここに問題があるのに気づきますか？：

```rust
// nothing after this line should fail
// この行以降は失敗することはない
<balances::Module<T> as Currency<_>>::transfer(&from, &to, value)?;
// but this function technically "could" fail
// しかしこの関数は正確にいうと、失敗する"かも"しれない
Self::transfer_from(owner.clone(), sender.clone(), kitty_id)?;
```

ということは、これはルール "まず検証せよ、書き込むのは最後だ" を破ったことになりますか？

ええと…正確には違います。この機能が失敗しないことを保証するために事前にチェックを行うことができれば、ランタイム開発のルールを破ったことにはなりません。

`transfer_from()`内で失敗する可能性がある箇所をもう一度チェックしてみましょう：

* そのキティに所有者は存在しない
* その"from"アカウントは問題のキティを所有できなかった
* そのユーザの `owned_kitty_count`からキティを引くとアンダーフローが発生する可能性がある
* そのキティをユーザの `owned_kitty_count`に追加するときオーバーフローが発生する可能性がある

よって、思うように関数を実装するには、これらすべてのチェックが成功することを確かめる必要があります。

1. `buy_kitty`関数では、そのキティに所有者がいることを確認する
2. その所有者の値をそのまま使い、`transfer_from()`関数を実行する
3. そのキティに所有者が存在する場合、その所有者のキティ所有数は0より大きいことを確認する
4. `all_kitties_count`と`owned_kitties_count`は、キティの数を追跡するために同じ型(`u64`）を使っていることを確認。新しいキティを作る唯一の方法である`mint()`関数は、`all_kitties_count`が決してオーバーフローを起こさないように確認。したがって、`owned_kitties_count`は`all_kitties_count`と同等かそれ以下であるはずなので、決してオーバーフローは発生しないということを自信を持って言えます。

私たちの`buy_kitty()`関数の場合は、なぜこの関数が失敗しないのかを証明するために`expect()`を実際に使うことができます：

``` rust
Self::transfer_from(owner.clone(), sender.clone(), kitty_id)
    .expect("`owner` is shown to own the kitty; \
    `owner` must have greater than 0 kitties, so transfer cannot cause underflow; \
    `all_kitty_count` shares the same type as `owned_kitty_count` \
    and minting ensure there won't ever be more than `max()` kitties, \
    which means transfer cannot cause an overflow; \
    qed");
```

実際に`expect`が[頻繁に使われている](https://github.com/paritytech/substrate/search?q=expect)ことをSubstrateレポ内で見ることができます。

コードとその中のロジックが正しく動くかを検証するのは、ブロックチェーン開発者であるあなたの義務です。Substrateは、他のスマートコントラクトプラットフォームのような、開発者にエラー保護を提供するフレームワークではありません。しかし、Substrateはブロックチェーン開発に最低限の機能を提供することにより、他モジュールとの圧倒的な拡張性とカスタマイズ性を実現しています。

## 演習してみよう！

既に提供されているテンプレートに従って`buy_kitty()`関数を完成させてください。そしてPolkadot-JS Apps UIで新たに追加した機能を実験してみてください。

<!-- tabs:start -->

#### ** テンプレート **

[embedded-code](../../3/assets/3.3-template.rs ':include :type=code embed-template')

#### ** 解答 **

[embedded-code-final](../../3/assets/3.3-finished-code.rs ':include :type=code embed-final')

<!-- tabs:end -->
