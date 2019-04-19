セットアップ
===

Substrateフレームワークを使えば、一行コマンドであなたのマシンに簡単に環境構築ができます：

```bash
curl https://getsubstrate.io -sSf | bash -s -- --fast
```
このスクリプトは、パッケージマネージャー、Rust、gitなどのようなさまざまなディペンデンシーをダウンロードしてインストールします。興味があれば、[URL](https://getsubstrate.io)にアクセスしてスクリプトを確認できます。

> `--fast`フラグを使うことにより、時間のかかる`substrate`の`cargo install`と`subkey`のインストールをスキップしていることに注意してください。これらはこのワークショップを完了するのに必要ありませんが、他のSubstrateチュートリアル/ガイドでは必要になるかもしれません。

このスクリプトでサポートされていないオペレーティングシステム（Windowsなど）で開発しようとしている場合は、[Substrateレポジトリ](https://github.com/paritytech/substrate#61-hacking-on-substrate)のセットアップ方法をご確認ください。

Substrateとそれに関連するディペンデンシー（Rustなど）のダウンロードとインストールを待っている間、他にもバンドルに含まれていないプログラムがあり、開発環境にインストールする必要があります:

 - [Node + NPM](https://nodejs.org/en/download/)
 - [Yarn](https://yarnpkg.com)
 - [Visual Studio Code](https://code.visualstudio.com/) または、好みのIDE
 - [Chrome](https://www.google.com/chrome/) またはChromiumベースブラウザ (Firefoxユーザーの皆さんごめんなさい！)

---

**発展**

UIはWebSocketを使用し、暗号化されていない接続を介してローカルのSubstrateノードインスタンスに接続します。ほとんどのブラウザはセキュリティとプライバシーの理由からこの種の接続を許可していません。Chromeのみがこの接続を許可しています（localhostに接続している場合）。このワークショップでChromeを使用しているのはそのためです。ネットワーク内の別のコンピュータを使用してブラウザに接続する場合は、安全な接続を介してサービスを受ける必要があります。

このワークショップで別のブラウザを使用する場合は、[Polkadot.jsアプリUI](https://github.com/polkadot-js/apps)をローカルにクローンして実行する必要があり、安全ではない接続が可能になります。

---
