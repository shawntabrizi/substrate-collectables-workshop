# 创建 Substrate UI

现在终于可以使用我们从 `substrate-package` 获得的 `substratekitties-ui` 文件夹了。这个文件夹包含一个来自 [`substrate-ui` repo](https://github.com/paritytech/substrate-ui/tree/substrate-node-template) 的 React 项目。

## 安装 Substrate UI

为了获得使用 Substrate UI repo 的最佳体验，你应该按照以下说明在你的计算机上安装 yarn：

[Install Yarn](https://yarnpkg.com/lang/en/docs/install/)

现在我们可以安装 UI 所需的 node 包了：

```
cd substratekitties-ui
yarn install
```

> **注意：** 如果在基于 debian 的系统上使用 `yarn` 时出错，请参阅底部的注释。

一旦完成包的安装后，你可以使用以下命令运行 UI：

```bash
yarn run dev
```

确保你的节点已启动并运行，并在 Chrome 中打开 [localhost:8000](http://localhost:8000)。

----
**Debug**

你可能需要删除现有 `yarn` 并通过 `npm` 安装：

```bash
sudo apt remove cmdtest
sudo apt remove yarn
sudo npm install -g yarn
```

`yarn install` 应该会按预期工作。

---
