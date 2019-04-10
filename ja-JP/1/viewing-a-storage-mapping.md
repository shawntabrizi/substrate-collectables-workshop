## Polkadot UIで作業内容を確認する

このコードはエラーなしでコンパイルされるはずですが、今は私たちの仕事をチェックするのに良い時期です。

## Checking Our Work in the Polkadot UI

Even though this code should compile without errors, now would be a good time to check out our work.

After running:

```bash
./build.sh
cargo build --release
./target/release/substratekitties purge-chain --dev
```

We can start our node:

```bash
./target/release/substratekitties --dev
```

[Polkadot-JS Apps UI]（https://polkadot.js.org/apps）に戻ると、ノードがブロックを生成しているという証拠が表示されるはずです。

If we go back into the [Polkadot-JS Apps UI](https://polkadot.js.org/apps), we should see evidence of our node producing blocks.

##トランザクションを送信する

** Extrinsics **アプリに行き、 "from extrinsic section"ドロップダウンを使って選択します。

## Submit a Transaction

Go to the **Extrinsics** app, and using the "from extrinsic section" dropdown select:

```
substratekitties > setValue(value)
```

Type in a value and press `Submit Transaction`:

![Submit a storage mapping in the Polkadot-JS Apps UI](./assets/submit-storage-mapping.png)

##ストレージを見る

値をストレージに入れるトランザクションを送信したので、値が実際にそこにあることを確認する必要があります。

**チェーン状態**アプリに移動して選択します。

## View the Storage

Now that you have submitted a transaction to put a value into storage, we should take a look that the value is actually there.

Go to the **Chain state** app and select:

```
kittyStorage > value(AccountId): u64
```

あなたがトランザクションを送信したアカウントのために、ストレージに問い合わせて青い `[+]`ボタンを押してください：

For the account you submitted the transaction with, query the storage and press the blue `[+]` button:

![Query for storage mapping](../../0/assets/view-storage-mapping.png)

あなたが保存したのと同じ値を取り戻すべきです！これを複数のアカウントで試してみると、各ユーザーが自分の値をランタイム記憶域に格納できることがわかります。

You should get back the same value you stored in! You can try this with multiple accounts and see that each user is able to store their own value into the runtime storage.
