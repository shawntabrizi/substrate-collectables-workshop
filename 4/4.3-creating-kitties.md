Creating Kitties
===

Now that we have gotten a taste for bonds and accessing runtime storage from our UI, let's actually build some interaction.

## Calling Our Runtime

In the last section we explored the `runtime` variable created by the Substrate UI. Now let's take a look at the `calls` variable:

```
calls.
```

![An image of the `calls` autocomplete](./assets/calls-autocomplete.png)

Again, we see that we automatically gain access to all of our modules, including the one we just created. Diving into the `substratekitties` we find:

```
calls.substratekitties.
```

![An image of `calls.substratekitties` autocomplete](./assets/calls-substratekitties-autocomplete.png)

This is a list of all the functions we created in the `decl_module!` macro. Note that our private functions like `_mint()` and `_transfer()` are not here since they are not supposed to be part of our public API.

## Making a Call

So let's try to make a call to create a new kitty. To do that, we can make a `post()` request to our runtime like so:

```javascript
post({
    sender: "5GoKvZWG5ZPYL1WUovuHW3zJBWBP5eT8CbqjdRY4Q6iMaDtZ",
    call: calls.substratekitties.createKitty()
}).tie(console.log)
```

In this sample, the `sender` is the address of one of our accounts. We can retrieve this for any of our accounts in the **Address Book** section of the Substrate UI:

![An image of the Address Book section](./assets/address-book.png)

When we submit this in our console, we will see a few things happen in the background, and then our transaction is `finalised` and the number of kitties increases:

![An image of creating a kitty from console](./assets/transaction-from-console.png)

## Creating a Transaction Button

Now that we know how to make a call to our runtime, we will want to integrate that into our UX. Again, we will take advantage of components provided to us by the Substrate UI called `TransactionButton` and `SignerBond`.

If you look at the code for the other sections on the page, you will find examples of how to integrate these parts.

The `SignerBond` creates an input field where a person can write the name of the account they want to sign some message. This account gets placed inside of a `bond`.

```javascript
this.account = new Bond;

<SignerBond bond={this.account}/>
```

You can then use this bond to power a `TransactButton`, where we will use the value stored in the bond to power the `sender` field of a transaction:

```javascript
<TransactButton
    content="Submit Transaction"
    icon='send'
    tx={{
        sender: runtime.indices.tryIndex(this.account),
        call: calls.myModule.myFunction()
    }}
/>
```

Because the `TransactionButton` has a dependency on `this.account`, it won't be active until the `SignerBond` has a valid input. Once it does, you can easily submit a transaction on behalf of that user.


## Other Components

We won't go deep into each component available through the Substrate UI. However, most of these components should be relatively easy to understand and reuse in your own sections.

There is very little magic happening beyond what is available in [React components](https://reactjs.org/docs/react-component.html), so that would be a good place to start to expand your knowledge.

## Your Turn!

Let's add a `Create Kitty` button to your Substrate UI.

You will need to create a new bond in your `constructor()`, and use that bond to power a `SignerBond` component.

Then you will want to connect that `SignerBond` to a `TransactionButton` which makes a `call` to `createKitty()`. You can use the `paw` icon to give your transaction button a little extra flare.

Once you have completed this, test your button by creating some new kitties! Watch your "kitty counter" increase as new kitties enter your system.

[embedded-code](./assets/4.3-template.js ':include :type=code embed-template')

<a href="javascript:toggleHint()" id="hint_link">Reveal the solution...</a>

[embedded-code-final](./assets/4.3-finished-code.js ':include :type=code embed-final')
