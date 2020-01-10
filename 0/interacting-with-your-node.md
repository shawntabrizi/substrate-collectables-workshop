Interacting With Your Node
===

By the end of this tutorial we will have you create your own custom UI to interact with your collectables blockchain. In the meantime, we can use the [Polkadot-JS](https://polkadot.js.org) Apps UI which is a generalized interface that adapts to your custom node.

In your web browser, navigate to:

https://polkadot.js.org/apps/

> Some browsers, notably Firefox, will not connect to a local node from an https website. An easy work around is to try another browser, like Chromium. Another option is the [host this interface locally](https://github.com/polkadot-js/apps#development).

To point the UI to your local node, you need to adjust the **Settings**. Just select 'Local Node (127.0.0.1:9944)' from the endpoint dropdown:

```
Settings > remote node/endpoint to connect to > Local Node (127.0.0.1:9944)
```

![An image of the settings in Polkadot-JS Apps UI](./assets/polkadot-js-settings.png)

After you press **Save and Reload**, you should notice the Polkadot-JS Apps UI come to life.
Note that you should have your `./target/release/substratekitties --dev` chain up and running while browsing the Polkadot UI.

Note that we will import a JSON file with additional type definitions later on in section "Registering a Custom Struct" of Section 1 > Viewing a Structure.

Let's go into the **Transfer** app, and make a transaction. The default account named "Alice" is pre-funded with a ton of *Units*.

Share some with "Bob" by creating a transaction. You should see a confirmation appear when the transaction has completed, and Bob's balance will also be updated.

![First Transfer in Polkadot-JS Apps UI](./assets/first-transfer.png)

Here you can see just how quickly we set up, ran, and interacted with our own local Substrate chain.

---
**Learn More**

The [Substrate UI](https://github.com/paritytech/substrate-ui) that has been built using the [oo7-substrate library](https://github.com/paritytech/oo7/tree/master/packages/oo7-substrate) is an alternative front-end interface to the [Polkadot-JS Apps UI](https://github.com/polkadot-js/apps) for interacting with your Substrate chain.

When building your own UI, you can refer to the [Polkadot-JS API Documentation](https://polkadot.js.org/api/) and the [oo7 API Documentation](https://paritytech.github.io/oo7/).
