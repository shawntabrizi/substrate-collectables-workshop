キティを追跡する
===

各ユーザーが独自の独自のキティを作成できるようになったので、次はそれらを追跡できるようにする必要があります。

私たちのゲームでは、作成されたキティの総数、および各キティを誰が所有しているかを追跡する必要があります

## "まず検証せよ、書き込むのは最後だ"

> 重要: このセクションは重要です。

Substrateの開発者として、ランタイムロジックを設計する方法と、Ethereumのようなプラットフォームでスマートコントラクトを開発する方法を区別する必要があります。

Ethereumの場合、トランザクションがどこかの時点で失敗したとしても（エラー、ガス切れなど）、スマートコントラクトのステートは影響を受けません。しかしながら、Substrateでは異なります。一度トランザクションが開始しブロックチェーンのストレージに変更を加え始めると、たとえトランザクションがランタイム実行中に失敗したとしても、それらの変更は永続的です。

これはブロックチェーンシステムには必要です。これはユーザーのナンス(Nonce)のようなものを追跡したり、発生した計算に対するガス代を差し引くことが必要になる場合があるためです。これらのことは両方とも実際には失敗したトランザクションに対するEthereumの状態遷移機能で発生しますが、これらを開発者として管理することについて心配する必要はありませんでした。

あなたがSubstrateランタイム開発者である今、あなたはブロックチェーンのステイトトランジションにより意識しなければならず、そしてそれが **"まず検証せよ、書き込むのは最後だ"** の法則に従わなければならない理由です。今回のチュートリアルでその方法を教えます。

## リストを作る

危険な習慣を導く可能性があるため、Substrateはネイティブでリスト型をサポートしていません。ランタイム開発では、リストの反復は悪と言われています。明示的に保護されていない限り、O(1)の料金のみを請求する操作に限りないO(N)の複雑さが加わります。その結果、あなたのチェーンは攻撃可能となる脆弱性を孕みます。そこで、以下のようにマッピングとカウンターでエミュレートすることで、リストの機能を代替することができます。

```rust
decl_storage! {
    trait Store for Module<T: Trait> as Example {
        AllPeopleArray get(person): map u32 => T::AccountId;
        AllPeopleCount get(num_of_people): u32;
    }
}
```

ここでは`AccountId`で表されるアカウントを`u32`でマッピングすることにより、リストを再現しています。これらのストレージ項目を正確かつ最新の状態に保つために、ストレージ項目を適切に取り扱うことに注意する必要があります。

## オーバーフロー/アンダーフローの確認

If you have developed on Ethereum, you may be familiar with all the problems that can happen if you do not perform "safe math". Overflows and underflows are an easy way to cause your runtime to panic or for your storage to get messed up.

Ethereum上での開発経験があるなら、"safe math"を使わないといけない問題を知っているかもしれません。オーバーフローとアンダーフローは、ランタイムをパニックにさせたり、ストレージを汚したりしてしまいます。

ストレージの状態を変更する前に、実行時エラーの可能性を常にチェックするようにしてください。Ethereumとは異なり、トランザクションが失敗した場合、状態はトランザクションの前に戻されないため、エラーに副作用がないことを確認するのは開発者の責任です。

幸いなことに、これらの種類のエラーのチェックは、基本的の数値型が[`checked_add()`](https://doc.rust-lang.org/std/primitive.u32.html#method.checked_add)と[`checked_sub()`](https://doc.rust-lang.org/std/primitive.u32.html#method.checked_sub)関数を持つRustでは非常に簡単です。

あるアイテムを`AllPeopleArray`に追加したいとしましょう。そうするにはまず、`AllPeopleCount`を問題なく増やすことができることを確認する必要があります：

```rust
let all_people_count = Self::num_of_people();

let new_all_people_count = all_people_count.checked_add(1).ok_or("Overflow adding a new person")?;
```

`ok_or`は以下と同等です:

```rust
let new_all_people_count = match all_people_count.checked_add(1) {
    Some (c) => c,
    None => return Err("Overflow adding a new person"),
};
```

`ok_or`は`match`よりも明確で読みやすいですが、文末の`?`を忘れないようにしてください。

オーバーフローが起こらずに`AllPeopleCount`を増やすことに成功したら、その更新された新しい値を`new_all_people_count`に代入します。問題が見つかった場合は、`Err()`を返します。エラーメッセージはブロック生成中のノードのコンソールに直接出力され、デバックを簡単に行うことができます。

## ストレージ内のリストを更新する

リストに安全に追加できることを確認したので、最後のストレージへの書き込みを行います。リストは0から数えるため、リストを更新するとき、「最後のインデックス」はカウントより1つ少ないことを覚えていてください。

人のリストに新しい人を追加する完全な例は次のようになります：

```rust
fn add_person(origin, new_person: T::AccountId) -> Result {
    let sender = ensure_signed(origin)?;

    let all_people_count = Self::num_of_friends();

    let new_all_people_count = all_people_count.checked_add(1).ok_or("Overflow adding a new person")?;

    <AllPeopleArray<T>>::insert(all_people_count, new_people);
    <AllPeopleCount<T>>::put(new_all_people_count);

    Ok(())
}
```

おそらくこの機能にも衝突検出を追加するべきです！その方法を覚えていますか？

## リストから削除する

この `map`と`count`パターンがもたらす一つの問題は、真ん中から要素を削除することで、リストに穴が開いてしまうことです。幸いにも、このチュートリアルで管理したいリストに順番は重要ではないのですが、この問題への解決法として"swap and pop"メソッドが考えられます。

"swap and pop"メソッドは削除したいアイテムの位置とリストの最後のアイテムを入れ替えます。そうすれば、リストに穴を開けずに最後の項目を簡単に削除できます。

アイテムを削除するたびに、削除するアイテムのインデックスを見つけるためにループを実行するのではなく、各アイテムとそのリスト内の位置を追跡するために、少し余分にストレージを使用します。

"swap and pop"のロジックについては後ほど詳しく紹介しますが、次の例のように`Index`ストレージを使って各項目のインデックスを追跡します：

```rust
AllPeopleIndex: map T::AccountId => u32;
```

これは実は`AllPeopleArray`の内容の逆です。このストレージアイテムは内部的に使用されており、モジュールAPIの一部として公開する必要はないため、ここではgetter関数は必要ありません。

## 演習してみよう!

すべてのキティを追跡するために何が必要かがわかったので、これを実装するためにランタイムを更新しましょう。

テンプレートに従って新しいストレージアイテムを追加し、新しいキティが作成されるたびに`create_kitty`関数を **"まず検証せよ、書き込むのは最後だ"** の法則にしたがって更新します。

<!-- tabs:start -->

#### ** テンプレート **

[embedded-code](../../2/assets/2.3-template.rs ':include :type=code embed-template')

#### ** 解答 **

[embedded-code-final](../../2/assets/2.3-finished-code.rs ':include :type=code embed-final')

<!-- tabs:end -->

---
**詳細解説**

クローンと借用について

[TODO: make this a page]

---
