Substrate UIをセットアップする
===

`substrate-package`から取得した`substratekitties-ui`フォルダを使用する時がやってきました。このフォルダには[`substrate-ui`リポジトリ](https://github.com/paritytech/substrate-ui/tree/substrate-node-template)からのReactプロジェクトが含まれています。

## Substrate UIを設定する

Substrate UIレポジトリを操作するために、まだ行っていなければコンピュータにパッケージマネジャーである`yarn`をインストールしてください：

[yarnをインストール](https://yarnpkg.com/lang/en/docs/install/)

yarnのインストールが完了すれば、UIに必要なノードパッケージをインストールできます：

```
cd substratekitties-ui
yarn install
```

>  **注意：** Debianベースのシステムで `yarn`を使っているときにエラーが発生した場合は、下のデバッグノートを見てください。

そして、パッケージのインストールが完了したら、次のようにしてUIを実行できます。

```
yarn run dev
```

サブストレートノードが起動して実行中であることを確認し、Chromeブラウザで[localhost：8000](http：//localhost：8000)を開きます。

----
**デバッグ**

既存の `yarn`（従来のコマンドラインツール）を削除してから、`npm`を使って新たにインストールする必要があるかもしれません：

```
sudo apt remove cmdtest
sudo apt remove yarn
sudo npm install -g yarn
```
`yarn install` が使えるようになっていると思います。

----

