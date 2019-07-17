Writing Tests
===

Now that we have our test mock set up, we're ready to write some unit tests for our runtime.

Let's start with a simple test that passes by default. Make sure to add a `#[test]` compiler flag followed by your test set up function. Let's use an `assert!(true)` inside the test for now to make it pass by default. 

Don't forget that the test must execute within the context of `TestExternalities`!

```rust
#[test]
fn it_works() {
	with_externalities(&mut build_ext(), || {
		assert!(true);
	})
}
```

The command for executing this test is:
`cargo test -p substratekitties-runtime substratekitties`

You can read this line as run the `substratekitties` unit test which is inside the `substratekitties-runtime` sub-package. This verbose specification is necessary because your runtime is nested inside of another Substrate package. 

Conversely, if want to test your entire runtime package, you can also run `cargo test -p substratekitties-runtime`.

The test should pass with the following console output: 

```zsh
running 1 tests
test template::tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Test "Create Kitty" Works

We've done the heavy lifting in setting up the test mock. From this point forward, it is quite easy to write the tests to interact with your modules.

So far, your Kitties module provides critical functionalities like the ability for anyone to create a kitty.

Let's write a test to check that `create_kitty_should_work`: 

```rust
#[test]
fn create_kitty_should_work() {
	with_externalities(&mut build_ext(), || {
		// create a kitty with account #10.
		assert_ok!(Kitties::create_kitty(Origin::signed(10)));

		// check that there is now 1 kitty in storage
		assert_eq!(Kitties::all_kitties_count(), 1);

		// check that account #10 owns 1 kitty
		assert_eq!(Kitties::owned_kitty_count(10), 1);
		
		// check that some random account #5 does not own a kitty
		assert_eq!(Kitties::owned_kitty_count(5), 0);

		// check that this kitty is specifically owned by account #10
		let hash = Kitties::kitty_by_index(0);
		assert_eq!(Kitties::owner_of(hash), Some(10));

		let other_hash = Kitties::kitty_of_owner_by_index((10, 0));
		assert_eq!(hash, other_hash);
	})
}
```

Notable things from the above snippet:

- Notice how using a `u64` as `AccountId` is handy. Your account is simply referred to as `10` rather than a special type.
- Notice how we've used the same `Kitties` to call a getter function (e.g. `kitty_of_owner_by_index`) and a normal dispatch function (e.g. `create_kitty` from `decl_module!`).
- Notice that you can use `Kitties` to call even the module functions as well (anything inside `impl Module<T>`)! Hence, you could (and should) have a test in which you only test the internal `mint()` function that is not directly accessible.

> **Note:** When accessing storage, you can also use the following alternative syntax in lieu of `Kitties::get()`: 

```rust
use super::KittyOwner;
use support::StorageMap; //Or: StorageValue, DoubleMap, etc. depending on the value type
assert_eq!(<KittyOwner<KittiesTest>>::get(hash), Some(10));
```

## Recommended Test Pattern

In Substrate, it is good practice to have comprehensive test coverage around your key state transition functions.

Recall that Substrate provides an `ensure!` macro used to check inputs and logic before updating runtime state. In particular, whenever this macro is used, you should implement thorough tests around any edge cases that may break the ensured assumptions. 

To aid you in writing comprehensive tests, the Substrate framework provides custom assert macros, in addition to the standard [assertion macros](https://doc.rust-lang.org/std/macro.assert.html) provided by Rust. 

You may want to frequently make use of: 
- [`assert_ok!()`](https://substrate.dev/rustdocs/v1.0/srml_support/macro.assert_ok.html): a special macro that checks a dispatch call returns an `Ok(())` Result. (Remember that dispatch calls return a special type of `Result<(), &'static str>`)
- [`assert_noop!()`](https://substrate.dev/rustdocs/v1.0/srml_support/macro.assert_noop.html): a special macro that checks that a call fails, whilst returning that particular error message string.

## Your Turn!

Now it is your turn. To complete this section, try writing tests for the following expectations:
  - Owner can successfully transfer a kitty.
  - A non-owner will fail to transfer a kitty. Specifically, make sure your test expectedly fails with an error message of: `"'from' account does not own this kitty"`

You are encouraged to write as many tests as you can at this point. 

## Challenge
After a few tests, you might notice that your tests require some common, manual setup (e.g. create 10 kitties for 10 accounts). Repeating this setup in each test is... well, not fun. 

Thankfully, __not repeating yourself__ is one of the main design intentions behind Substrate and there is a way to get around it.

Our challenge to the reader is to extend the test functionality by giving Kitties runtime a `genesis config`, which allows you to preconfigure the state of the chain before the first block. A genesis config is useful in scenarios when we want to initialize the chain to have certain parameters for subsequent transactions. In this case, it could be helpful to have some initial kitties to start the game and to make subsequent testing easier.

You can see another sample tutorial which implements the genesis config [here](https://substrate.dev/docs/en/tutorials/tcr/building-the-substrate-tcr-runtime#using-the-genesis-config).

<!-- tabs:start -->

#### ** Template **

[embedded-code](./assets/5.2-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](./assets/5.2-finished-code.rs ':include :type=code embed-final')

<!-- tabs:end -->