Breeding a Kitty
===

Probably the most unique part of the original CryptoKitties game is the ability to breed new kitties from existing ones.

We have prepared our `Kitty` object with this in mind, introducing `dna` and `gen` which will be used in forming brand new kitty offspring.

## Passing Down Traits

In our UI, we will be generating our kitties image using the kitty DNA. In our runtime, the DNA is a 256 bit hash, which is represented by as a bytearray in our code, and a hexadecimal string in our upcoming UI.

This means that there are 32 elements, each of which can be a value from 0 - 255. We will use these elements to determine which traits our kitties have. For example, the first index of the byte array can determine the color of our kitty (from a range of 256 colors); the next element could represent the eye shape, etc...

```
Attribute:  Color Eyes Hair Collar Accessory
DNA:        [233] [15] [166] [113] [67] ...
```

When we breed two kitties, we want the offspring to have some combination of the parent's genetics. What we will do for our game is randomly choose one of the parents to give their attribute to the child kitty.

```
Kitty1 DNA:   [212] [163] [106] [250] [251] [  0] [ 75]...
                |     |                       |
Child DNA:    [212] [163] [ 69] [195] [223] [  0] [201]
                            |     |     |           |
Kitty2 DNA:   [233] [ 49] [ 69] [195] [223] [133] [201]...
```

We provide you with the gene splicing algorithm in your template, but feel free to make modifications.

## Chain Upgrade (Optional)

One of the cool features of Substrate is the ability to do forkless, real-time chain upgrades which introduce new features and functionality to your blockchain.

Rather than purging your chain and rebuilding the native binaries to introduce the `breed_kitty()` function, we will show you how you can use just the Wasm runtime to upgrade your chain.

Keep your node running from the previous section, and in a new terminal run just:

```
./build.sh
```

This will generate a new compact Wasm binary in the following path:

```
./runtime/wasm/target/wasm32-unknown-unknown/release/substratekitties_runtime.compact.wasm
```

[TODO: finish upgrade]

[embedded-code](./assets/3.4-template.rs ':include :type=code embed-template')

<a href="javascript:toggleHint()" id="hint_link">Reveal the solution...</a>

[embedded-code-final](./assets/3.4-finished-code.rs ':include :type=code embed-final')