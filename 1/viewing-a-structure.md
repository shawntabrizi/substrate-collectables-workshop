Viewing a Structure
===

Now that we have set up our runtime to make kitties, we should check our work!

We have introduced a custom structure to our chain, and while the Polkadot-JS Apps UI is very good at adapting to our changes, in this situation, we need to give it a hint on how to deserialize our structured data.

> REMINDER: Remember to reset your chain so that you start of fresh when interacting with the UI:
>
> ```
> ./scripts/build.sh
> cargo build --release
> ./target/release/substratekitties purge-chain --dev
> ./target/release/substratekitties --dev
> ```

## Registering a Custom Struct

Fortunately, the Polkadot-JS Apps UI provides us with a very simple way to import custom structures so that the page will be able to decode the information correctly.

Go back to the **Settings** app. Under the **Developer** section, you can either submit a JSON file with your custom structure or add it manually through a code editor. Copy & paste this JSON object into the code editor and press `Save`.

```
{
    "Kitty": {
        "id": "H256",
        "dna": "H256",
        "price": "Balance",
        "gen": "u64"
    }
}
```

## Creating a Kitty

Now we can go and create a new kitty. In the **Extrinsics** app, go to:

```
substratekitties > createKitty()
```

Once you press submit, you should see the transaction finalize:

![Image of creating a kitty in the Polkadot-JS Apps UI](./assets/creating-a-kitty.png)

## Viewing a Kitty

Finally, we can go into the **Chain State** app and view our stored kitty object. Select:

```
kittyStorage > ownedKitty(AccountId): Kitty
```

Then select a user who has called the `createKitty()` function. You should then be able to see the individual properties of the `Kitty` object:

![Image of viewing a kitty object in the Polkadot UI](./assets/view-kitty.png)

---
**Learn More**

Talk about serialization and deserialization.

How we simply transfer and transmit raw bytes

[TODO: make this a page]

---
