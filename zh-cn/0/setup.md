# 安装

Substrate 框架提供了一组简单的命令来设置本地环境:

```bash
curl https://getsubstrate.io -sSf | bash -s -- --fast
```

这个脚本将会下载并安装 Rust 开发环境，git 等等。如果你感兴趣，可以通过 [链接](https://getsubstrate.io) 查看脚本。

> 注意我们使用 `--fast` 参数跳过 `substrate` 和 `subkey` 的安装过程，因为这个过程比较花时间，而且不是本教程所必需的，但你可能会在其他 Substate 教程使用到。

如果你尝试在不支持该脚本的操作系统（比如 Windows）上进行开发，你可以看看 [Substrate 仓库](https://github.com/paritytech/substrate#61-hacking-on-substrate) 中的安装说明。

在等待 Substrate 及其相关依赖项（如 Rust 等）下载和安装的同时，还需要在你的开发环境中安装其他可能未包含在包中的程序：

- [Node + NPM](https://nodejs.org/en/download/)
- [Visual Studio Code](https://code.visualstudio.com/) (或者你喜欢的 IDE)
- [Chrome](https://www.google.com/chrome/) 或者基于 Chromium 的浏览器 (抱歉 Firefox 用户 :[ )

---

**Learn More**

UI 使用 WebSockets，通过一个未加密的连接来连接本地 Substrate 节点。绝大多数浏览器出于安全和隐私原因不允许这种连接，只有 Chrome 连接 localhost 时才允许这种连接。这就是我们在本教程中使用 Chrome 的原因。如果你想使用网络中其他计算机连接浏览器，则必须通过安全连接才行。

如果你想在本教程中使用其他的浏览器，你将需要 clone 并且在本地运行 [Polkadot.js Apps UI](https://github.com/polkadot-js/apps)，以便让 UI 可以在不安全连接下使用。