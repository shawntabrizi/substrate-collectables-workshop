# [Substrate Collectables][main link]
> インタラクティブハンズオンではじめてのブロックチェーンを作る[Substrate][] ワークショップ

![A screenshot of Substrate kitties](./media/substrate-collectables.png)

## これは何?

これは自分のペースで進められるインタラクティブハンズオンワークショップです。
ここでは、[Substrate][]と、[Parity][]によるオープンソース[Rust][]ブロックチェーン開発キット使った初めてのブロックチェーンの作り方を学べます。
ワークショップレッスンを通して、コレクタブルブロックチェーン（アセットを作り、その所有権を管理したり送ったりできるチェーン）を作ります。

そのため、この資料では上記チェーンを構築する論理に焦点を当てます。ブロックチェーンのネットワーキング、コンセンサス、または経済的インセンティブの側面については説明しません。幸いなことに、Substrateには素晴らしいネットワーキングとコンセンサスエンジンが組み込まれているので、チェーンロジックだけに集中できます。

基板は現代の静的型システムプログラミング言語[Rust] []を使って作られています。このワークショップでは、言語の詳細には触れません。この言語は読みやすく、追うのが非常に簡単です。プログラム経験があればたとえ[Rust][]初心者でも、ある程度たどって、ワークショップを終えるのにそれほど苦労しないはずです。

## これはどうやってやるの?

それぞれの章を一つずつ進め、一度に一つのエクササイズをしてください。この資料は、あなたが自分でできるようにすることを目的としていますが、学習グループやホステッドワークショップで、一緒に集まって他の人と共同で作業することを強くお勧めします。時々動かなくなったり、資料が何を説明しようとしているのか理解できなくなったりするのはまったく普通のことです。そのような問題を解決するために周りに同士を持つことはとても役に立ちます。それとは別に、[資料に関するフィードバック、行き詰まりやすい場所](feedback)も非常に役に立つでしょう。

# [やってみよう!](/0/introduction.md)

---
**注意**

サブストレートは急速に発展しているプロジェクトです。つまり、新しい変更によりこのワークショップの指示通りに従おうとすると問題が発生する可能性があります。どんな問題でも[お気軽にお問い合わせください](https://substrate.readme.io/v1.0.0/docs/feedback)。

---

## 貢献の仕方

* [issues on our GitHub](https://github.com/shawntabrizi/substrate-collectables-workshop/issues)を開く

* [repository](https://github.com/shawntabrizi/substrate-collectables-workshop)のフォークとクローン

* [Visual Studio Code](https://code.visualstudio.com/)と[Live Server](https://marketplace.visualstudio.com/items?itemName=ritwickdey.LiveServer)のようなツールを使い、ローカルサーバー環境を立ち上げる。

*変更をコミットしてオリジンフォークのブランチにプッシュし、上流のリポジトリへのプルリクエストを作成することで貢献します

##謝辞

サブストレートやこのワークショップのようなオープンソースプロジェクトは、開発者コミュニティの総意と共同作業なしには成功しません。

Substratekittiesワークショップが提供できるのは[Cryptokitties]（https://www.cryptokitties.co/）、[Cryptozombies]（https://cryptozombies.io/）、[Docsify]（https：// docsify.js.org/）、[モナコエディタ]（https://microsoft.github.io/monaco-editor/）、[David Revoyの猫用アバタージェネレータ]（https://framagit.org/Deevad/cat-avatar-generator)のようなジャイアント達と、途中でエラーやバグを報告してくれた多数のボランティアのお陰です。

このワークショップがあなたに何か新しいことを教え、その代わり、あなたが他の人に教えてくれることを願います。

---

[main link]: https://shawntabrizi.github.io/substrate-collectables-workshop/
[feedback]: https://docs.substrate.dev/docs/feedback
[Substrate]: https://www.parity.io/substrate/
[Substrate docs]: https://docs.substrate.dev/
[Parity]: https://www.parity.io/
[Rust]: https://www.rust-lang.org/