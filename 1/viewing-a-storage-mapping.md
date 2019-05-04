## Checking Our Work in the Polkadot UI

Even though this code should compile without errors, now would be a good time to check out our work.

After running:

```bash
./scripts/build.sh
cargo build --release
./target/release/substratekitties purge-chain --dev
```

We can start our node:

```bash
./target/release/substratekitties --dev
```

If we go back into the [Polkadot-JS Apps UI](https://polkadot.js.org/apps), we should see evidence of our node producing blocks.

## Submit a Transaction

Go to the **Extrinsics** app, and using the "from extrinsic section" dropdown select:

```
substratekitties > setValue(value)
```

Type in a value and press `Submit Transaction`:

![Submit a storage mapping in the Polkadot-JS Apps UI](./assets/submit-storage-mapping.png)

## View the Storage

Now that you have submitted a transaction to put a value into storage, we should take a look that the value is actually there.

Go to the **Chain state** app and select:

```
kittyStorage > value(AccountId): u64
```

For the account you submitted the transaction with, query the storage and press the blue `[+]` button:

![Query for storage mapping](./assets/view-storage-mapping.png)

You should get back the same value you stored in! You can try this with multiple accounts and see that each user is able to store their own value into the runtime storage.
