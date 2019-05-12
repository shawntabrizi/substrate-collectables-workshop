複数のキティを所有する
===

あなたは現在のランタイムに問題があることには気づきましたか？同じユーザーが `create_kitty()`関数を呼び出した場合どうなると思いますか？

今のところ、私たちのストレージは1ユーザーあたり1つのキティしか追跡することができませんが、キティの数は増え続けるでしょう！ユーザーが複数のキティを所有できるようにすることでその問題に対処します！

## 高次配列をエミュレートするためのタプルの使用

複数のユーザーにわたる複数のアイテムの所有権を表すために、より複雑なストレージアイテムをいくつか導入する必要があります。

`AccountId`と`Index`の[tuple](https://doc.rust-lang.org/rust-by-example/primitives/tuples.html)を使うことで、私たちのニーズを満たすことができます。

この構造を利用することで、各自に固有の「友達リスト」を作成することもできます：

```rust
MyFriendsArray get(my_friends_array): map (T::AccountId, u32) => T::AccountId;
MyFriendsCount get(my_friends_count): map T::AccountId => u32;
```

このケースではもっと標準の２次元配列をエミュレートした方がいいでしょう；

```rust
MyFriendsArray[AccountId][Index] -> AccountId
```

ユーザごとの友達の番号は以下のように取得できます；

```rust
MyFriendsArray[AccountId].length()
```

## 相対的インデックス

以前と同様に、アイテムの位置をインデックス化することで、ランタイムが行う必要がある計算作業を最適化できます。これに対する一般的なアプローチは、 `MyFriendsArray`のマッピングを逆にして、次のようなストレージアイテムを作成することです：

```rust
MyFriendsIndex: map (T::AccountId, T::AccountId) => u32;
```

一つ目の`AccountId`がユーザー、二つ目が友人を表します。
この戻り値はその友人が保存されている`MyFriendsArray`のインデックスになります。

しかし、私たちのキティはすべて `Hash`として一意の識別子(`dna`)を持ち、複数のユーザが所有することはできないので、実際にはこの構造を単純化することができます。

```rust
MyKittiesIndex: map T::Hash => u32;
```

戻り値はその`Hash`を持つキティの **所有者** 配列内の、そのキティのインデックスになります。

## 演習してみよう!

テンプレートに従って最後のストレージアイテムを導入し、`create_kitty()`関数がこれらのストレージアイテムを正しく管理していることを確認してください。前回のセクションと非常によく似ているので、それほど難しくはないはずです。

<!-- tabs:start -->

#### ** テンプレート **

[embedded-code](../../2/assets/2.4-template.rs ':include :type=code embed-template')

#### ** 解答 **

[embedded-code-final](../../2/assets/2.4-finished-code.rs ':include :type=code embed-final')

<!-- tabs:end -->

---
**発展**

<T::AccountID>について

[TODO: make this a page]

---
