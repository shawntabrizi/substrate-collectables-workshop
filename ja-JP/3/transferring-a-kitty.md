キティを送る
===

前回はキティの値段を更新できるようにする関数を実装しましたが、キティを買うには、所有権を転送する関数も必要です。

ユーザーは複数のキティを所有をすることができるので、1人のユーザーが自分の所有するキティを別のユーザーに送ることができる機能を追加することもできます。

所有権はストレージによって完全に管理されているので、`transfer_kitty`関数を実装するには、既存のストレージを変更して新しい状態を反映するだけです。

更新する必要があるストレージ項目について考えてみましょう。

 - グローバルキティの所有者を変更する
 - 各ユーザーのowned_kitty_countを変更する
 - owned_kitty_indexを変更する
 - 各ユーザーの所有キティマップを変更する

## スワップ＆ポップ

このチュートリアルの前半で、「スワップとポップ」メソッドを使用して、穴を開けることなく項目を削除する方法を説明しました。このメソッドはリストの順番が重要な場合はアルゴリズムの再考が必要ですが、私たちの場合は非常に便利なメソッドでしょう：

```rust
let kitty_index = <OwnedKittiesIndex<T>>::get(kitty_id);

if kitty_index != new_owned_kitty_count_from {
    let last_kitty_id = <OwnedKittiesArray<T>>::get((from.clone(), new_owned_kitty_count_from));
    <OwnedKittiesArray<T>>::insert((from.clone(), kitty_index), last_kitty_id);
    <OwnedKittiesIndex<T>>::insert(last_kitty_id, kitty_index);
}
```

削除したいインデックスが配列の最後の要素になっていない場合にのみ「スワップ」を行う必要があります。そのような状況では、「ポップ」だけが必要です。

## あなたの番です！

キティを送る機能を実装するには、これまでに学んだツールを使用する必要がありますが、新しいことは何もありません。

転送機能は他のモジュールも再利用可能にしたいので、その設計テンプレートは既に提供されています。

パブリック関数である`transfer()`には、プライベート関数の`transfer_from()`が必要なので、テンプレートに従って実装してください。

<!-- tabs:start -->

#### ** Template **

[embedded-code](../../3/assets/3.2-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](../../3/assets/3.2-finished-code.rs ':include :type=code embed-final')

<!-- tabs:end -->