イントロダクション
===

[CryptoKitties](https://www.cryptokitties.co/)は、ご存知の方も多いとは思いますが、ピーク時にはEthereumブロックチェーン上の全トランザクションの20％以上を占めていたこともある、超人気Ethereum dAppです。このチュートリアルでは、dAppの王者でもあるCryptoKittiesの足跡をたどり、Substrate上にネクストヒットとなるdAppを作ります。

このセクションでは、ユーザーが非代替トークン(non-fungible token)を作成および所有できるようにするランタイムの作成方法を説明します。

## Non-Fungible Token(NFT)

CryptoKittiesおよび他の同様のdAppは、資産のユニーク性を表すために**NFT**を使用します。非代替トークン（NFT）は、ユニークで区別可能なトークンです。Bitcoinは他の人のBitcoinと交換しても特に違いはありませんが、CryptoKittiesでキティを作成すると、決して代替不可能な世界に一つだけの種類のアイテムが生成されます。

NFTは、それらがユニークであるという理由だけではなく、それらを使って可能になることがたくさん思いつきます。あなたは、トークンを所有、交換、購入または売却、トークンの履歴を追跡、そしてトークン同士をインタラクさせることさえもできます。

## ERC-721

[ERC-721](http://erc721.org/)は、NFTの実装に人気のあるEthereumトークン標準です。この標準は既存のスマートコントラクトに広く採用されているため、私たちも同じようにアプリケーションの構造化方法の基礎としてERC-721を使用します。

具体的には、[OpenZeppelinの実装](https://github.com/OpenZeppelin/openzeppelin-solidity/blob/master/contracts/token/ERC721/ERC721.sol)を使うことにより、トークン承認などの必要ない機能を除外します。