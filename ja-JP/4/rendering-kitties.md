Rendering Kitties
===

If you made it this far, then you have truly earned what we are about to do in this section: Add kitties to our UI.

## Adding Our Custom `KittyCard` Component

We have built a custom React component for showing kitties. It is not so complicated, but for the purposes of this workshop, we will not have you build it. You can download the component as a `.zip` [here](https://github.com/shawntabrizi/substrate-collectables-workshop/raw/master/4/assets/KittyCards.zip).

To add it, you must place the `KittyCards` folder in your `src` folder:

```
substratekitties-ui/
|
+-- src/
    |
    +-- KittyCards/      <-- Place here
    |
    +-- AccountIdBond.jsx
    |
    +-- ...
```

Then, inside the main `app.jsx` file you need to import this component:

```javascript
import { KittyCards } from './KittyCards';
```

This will give you access to the `<KittyCards>` component:

```
<KittyCards count={runtime.substratekitties.allKittiesCount} />
```

### Add the `Kitty` Type

Before this component will work, we need to tell the Substrate UI about our custom `Kitty` type. We can do that with the `addCodecTransform()` function made available to us by the oo7 JavaScript library.

In the case of our kitty object, it would look like this:

```
addCodecTransform('Kitty<Hash,Balance>', { 
    id: 'Hash',
    dna: 'Hash',
    price: 'Balance',
    gen: 'u64'
});
```

We can add this to our application's `constructor()` function to ensure it gets loaded at the start. After this, we can interact with the attributes of the `Kitty` object like we could any other JSON object.

> Note: The codec transform uses a key/value pair to look up the object structure that should be used for deserialization. As a result, it is important that your "object name" matches exactly what is expected. In this situation, note there are no spaces in the object name `Kitty<Hash,Balance>`. If the Substrate UI cannot find the right key for a custom object, it will give you an error in your browser console with the exact object name it is expecting.

## Peek Inside Our Custom Component

Since we won't have you build this component, we will show you some of the working pieces instead.

![An image of the kitty cards in the UI](./assets/kitty-cards.png)

### Dynamic Card Loading

Let's quickly walk through the parts of `/KittyCards/index.jsx` to show how we are able to dynamically load new cards when kitties are added to the system.

We call the `KittyCard` component from our UI with:

```
<KittyCards count={runtime.substratekitties.allKittiesCount} />
```

This component is tied to the `allKittiesCount` bond, that will automatically update as the state of our blockchain changes.

When `allKittiesCount` changes, the `readyRender()` part of our `KittyCards` component triggers, which grabs the latest count, and loops over each item in the `allKittiesArray()` which will return the unique kitty id `Hash`.

```javascript
    readyRender() {
        let kitties = [];

        for (var i=0; i < this.state.count; i++){
            kitties.push(
                <div className="column" key={i}>
                    <KittyWrap hash={runtime.substratekitties.allKittiesArray(i)} />
                </div>
            );
        }
```

This then sends the kitty id `Hash` to the `KittyWrap` component which does a simple lookup for the `owner` and the `Kitty` object. If the `hash` sent to `KittyWrap` doesn't change between loops, then React will simply skip the re-rendering process.

```javascript
class KittyWrap extends ReactiveComponent {
    constructor(props) {
        super(['hash']);
    }

    readyRender() {
        const { hash } = this.state; // Object destructuring assignment syntax

        return (
            <KittyCard
                kitty={runtime.substratekitties.kitties(hash)}
                owner={runtime.substratekitties.kittyOwner(hash)}
            />
        );
    }
}
```

Finally, `KittyWrap` calls `KittyCard` which actually produces the contents of each card.

Note that we have used JavaScript [Object Destructuring Assignment Syntax](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Destructuring_assignment#Object_destructuring) to unpack the `hash` property from
`this.state` since we are reusing the value multiple times (rather than repeatedly having to write `this.state.hash`).

### Card Contents

Our `KittyCard` component takes the `Kitty` object passed from `KittyWrap`, as well as the owner, and formats all data. From top to bottom:

- The Kitty ID
- The Kitty DNA (which is the same for kitties that are gen 0)
- The Kitty Owner
- The Kitty Generation
- The Kitty Price

```javascript
<Card>
    <KittyAvatar dna={kitty.dna} />
    <Card.Content>
        <Card.Header>
            <Pretty
                value={kitty.id}
                className='limit-name'
            />
        </Card.Header>
        <Card.Meta>
            <Pretty
                value={kitty.dna}
                className='limit-dna'
            />
        </Card.Meta>
        <Rspan>
            <b>Owner</b>: {
                secretStore()
                    .find(this.state.owner)
                    .name
            }
        </Rspan>
        &nbsp;
        <Identicon
            key={this.state.owner}
            account={this.state.owner}
            size={16}
        />
        <br />
        <Rspan>
            <b>Generation</b>: {kitty.gen}
        </Rspan>
        <br />
    </Card.Content>
    <Card.Content extra>
        <Pretty
            value={kitty.price}
            prefix="$"
        />
    </Card.Content>
</Card>;
```

You can see we are using bonds and other Substrate UI components throughout to make the final render.

### Generating Kitties from DNA

You can see that our `KittyCard` has one more layer of abstraction in the `<KittyAvatar>` component.

If you look in the `/KittyCards/avatars/index.jsx you will see a final React component which manages the generation of the kitty images.

The core function in the component is `dnaToAttributes()`:

```
function dnaToAttributes(dna) {
    let attribute = (index, options) => dna[index] % options;

    return {
        body: IMAGES.body[attribute(0, 15)],
        eyes: IMAGES.eyes[attribute(1, 15)],
        accessory: IMAGES.accessories[attribute(2, 20)],
        fur: IMAGES.fur[attribute(3, 10)],
        mouth: IMAGES.mouth[attribute(4, 10)]
    };
} 
```

The kitty DNA is an array of bytes (0-256), and we map the first 5 bytes to control the specific attribute chosen for a kitty. Not only can you see that we can have 256 configurable options per attribute, but we can have up to 256 attributes too!

We used the [Cat Avatar Generator](https://framagit.org/Deevad/cat-avatar-generator/tree/master) project by David Revoy to power our images, but the sky is the limit if you can find a good artist.

## Your Turn!

Now that you understand how the `KittyCard` component works, it's time to integrate it into your Substrate UI. This should be trivial since the React component does all of the hard work.

After you are done, try creating a kitty and watch your UI update! You can also try out your other Runtime functions by initiating the transaction in the Polkadot UI in a different tab.

Do you want to test your React + JavaScript + Bonds skills? We have only integrated one function of our runtime, but there are plenty more that you can try to build:

- Setting the price of a kitty
- Buying a kitty
- Showing the kitties for a specific owner
- Breeding kitties
- etc...

Also think about how you might need to change the `KittyCards` component if you introduce a way to remove kitties from your runtime (setting them free!).

<!-- tabs:start -->

#### ** Template **

[embedded-code](../../4/assets/4.4-template.js ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](../../4/assets/4.4-finished-code.js ':include :type=code embed-final')

<!-- tabs:end -->
