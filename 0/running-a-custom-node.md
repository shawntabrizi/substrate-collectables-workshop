Running a Custom Node
===

Now that you have successfully installed all of the prerequisites on your machine, we can quickly spin up a custom Substrate node using a pre-configured template.

Substrate is a rapidly evolving project, which means that breaking changes may be introduced from time to time. In order to improve the development experience in this workshop, we have created a stable, known working version of a Substrate node and a compatible Substrate UI which you will use for this tutorial.

As long as you start with this Substrate package, you should be able to complete the rest of this tutorial without issue, but please let us know if that is not the case. To get the package, run the following command in your working directory:

```bash
git clone https://github.com/shawntabrizi/substrate-package
```

The `substrate-package` repository consists of two folder:

1. `substrate-node-template`
2. `substrate-ui`

We won't touch the `substrate-ui` folder until Chapter 4 of this workshop, but as the name implies, it includes a pre-built UI, written in [React](https://reactjs.org/), which can later be extended for custom experiences.

Instead, we will primarily be working in the `substrate-node-template` folder which contains a minimal, working Substrate node which we will start to hack on top of.

Let's rename our project and project folders using the `substrate-package-rename.sh` script:

```bash
./substrate-package-rename.sh substratekitties <your_name>
```

Then let's go into the now renamed `substratekitties` folder and build our pre-configured node:

```bash
cd substratekitties
./init.sh
./build.sh
cargo build --release
```

This process may take a little while, but once it is done, you should be able to start your node with:

```bash
./target/release/substratekitties --dev
```

If you've done everything right so far, you should see blocks being produced.

![An image of the node producing new blocks](./assets/building-blocks.png)

Nice, you just started your own custom blockchain!

---
**Learn More**

The `substrate-package` repository is made using [some custom commands](https://github.com/paritytech/substrate-up) provided by the `getsubstrate.io` one-liner we ran earlier.

If you are starting a new project and want to get the latest version of Substrate, you can build your own Substrate package by running:

```bash
substrate-node-new <project_name> <your_name>
substrate-ui-new <project_name>
```

As mentioned earlier, the one downside of this method is that these scripts pull directly from different GitHub repositories, which means there may be times of incompatibility during breaking changes.

---
