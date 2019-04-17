# 常见模式

## Rust 编译器是你的朋友

使用强类型编程语言（如 Rust）的众多优点之一

[TODO: 让 RUST 成为你的朋友，在需要时帮助你添加代码]

## 更新你的 Runtime

在我们开始创建自定义 Substrate runtime 之前，你应该熟悉一些可以帮助你迭代和运行代码的模式。

你的 Substrate runtime 代码被编译为两个版本：

- WebAssembly（Wasm）image
- 标准二进制可执行文件

Wasm 文件用作标准二进制文件编译的一部分，因此在构建可执行文件之前需要先编译 Wasm image。

模式应该是：

```bash
./build.sh               // Build Wasm
cargo build --release    // Build binary
```

此外，当你对节点进行更改时，之前旧版本节点生成的块仍然存在。你可能会注意到，当重启节点时，块只会从中断处继续生成。

但是，如果你对 runtime 的更改很重要，那么可能需要使用以下命令清除链上先前所有的块：

```bash
./target/release/substratekitties purge-chain --dev
```

完成所有这些后，你将能够再次启动带有最近更改的节点：

```bash
./target/release/substratekitties --dev
```

记住这种模式; 你会经常使用它。

---

**Learn More**

你总是应该使用最近的 Rust stable 和 nightly 版本来在 Substrate 上开发。

我们在 `build.sh` 所在的目录中提供了另一个脚本，你应该在每次启动一个新项目时运行它：

```bash
./init.sh
```

此脚本只是简单更新 Rust 版本和相关工具。确保你没有奇怪的编译错误。如果你有相关问题，请记得联系我们，具体方式我们已经在本教程的 instructions 部分提到过了。

---