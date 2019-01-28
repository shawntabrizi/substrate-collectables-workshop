Running a Custom Node
===

Now that you have successfully installed the Substrate framework on your machine, we can quickly spin up a custom Substrate node using pre-configured templates.

In your terminal window, navigate to your working directory and run `substrate-node-new --help` to view usage instructions. We will choose "substratekitties" as the name of our project for the purposes of this walkthrough, but you can modify this name for future projects you work on. Now run the following, where `<AUTHOR>` is your first name:

```bash
substrate-node-new substratekitties <AUTHOR>
```

> **NOTE**: If you want to peek behind the magic of `substrate-node-new` you can take a look [here](https://github.com/paritytech/substrate-up/blob/master/substrate-node-new).

Once your custom node is done compiling, you should be able to run the following command to start the node:

```bash
cd substratekitties
./target/release/substratekitties --dev
```

If you are successful, you should see blocks being produced.

![An image of the node producing new blocks](./assets/building-blocks.png)

---
**Learn More**

Using the `--dev` flag tells your node binaries to run a specific chain 

---