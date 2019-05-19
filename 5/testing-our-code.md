Testing Our Code
===

Say that you have built your awesome module (kitties or whatever) and already gave it a challenge through the UI. While that's quite interesting to do and see in action, it is not merely enough. Tests are the backbones of any software artifact, ensuring its veracity as it stands. More importantly, having solid tests will help you ensure that you are not breaking previous capabilities while not breaking any of the previous ones. Obviously, testing manually through the UI is not an option, namely because:

- It is __manual!__.
- Even if you automate it with some end-to-end testing tools it is more than your runtime code. Frontend errors might be mistaken for rust codes etc.

What we want is to merely test the _logic_ that we have placed in out runtime code.

Thankfully, substrate has some low-level primitives that help you execute your runtime with minimum hassle, meanwhile giving it capabilities such as calling other modules and processing blocks to be as realistic as possible.

Without further do, let's dive into the template some code. A huge portion of your template for this part is boilerplate needed for setup, so let's first look at some of the more important pieces:

We will first begin with importing some modules into the newly created `test` module.

```rust
use support::{impl_outer_origin, assert_ok};
use runtime_io::{with_externalities, TestExternalities};
use primitives::{H256, Blake2Hasher};
use runtime_primitives::{
	BuildStorage, traits::{BlakeTwo256, IdentityLookup},
	testing::{Digest, DigestItem, Header}
};
```

Most of these are needed to replace the config types of the traits that we want to implement. From the above imports, the ones from `runtime_io` will be more important.

- From the [definition of `TestExternalities`](https://crates.parity.io/sr_io/struct.TestExternalities.html) we can see that it is an in-memory, hashmap-based, externalities. In other words, it provides the storage needed for the runtime to execute in a minimal fashion. It also accepts the a generic type of `Hasher`, hence we have imported the `Blake2Hasher` to use it later on while building `TestExternalities`.
- From the definition of the `with_externalities` function we can see that it accepts two arguments, namely:
  - an object of type `Externalities`.
  - a closure that is being executed given the first argument.

From the above we can imagine that the basic structure of a runtime test is:

```rust
with_externalities(some_externality, || {
	some_assertions!()
})
```

Which is percisely how it actually works.

Moving on, the rest of the code will look extreamly similar to how your `lib.rs` is written in this tutorial. We create an empty struct ( `KittiesTest`) and implement all of the needed traits for it. We will not focus on the values being passed into each trait in favor of brevity. Yet, you should note that we are keeping everything as simple as possible, namely:

- `AccountId` is now simply a number, `u64`.
- All events are given the unit type, `()`.

The main layout of this part looks like this:

```rust
impl_outer_origin! {
	pub enum Origin for KittiesTest {}
}

#[derive(Clone, Eq, PartialEq)]
pub struct KittiesTest;

impl system::Trait for KittiesTest {
	...
}
impl balances::Trait for KittiesTest {
	...
}
impl super::Trait for KittiesTest {
	...
}
```

You can see how we progress by extending the dummy struct (`KittiesTest`) by adding trait implementations to it. Of course, this ends with implementing the trait of our own module, `super::Trait`.

Next, we create some type aliases to help us access this module from this point onward. All modules are built in a way that the `Module` struct wraps all functions attached to it (both in `decl_module!` and `impl Module<T>`).

```rust
type Kitties = super::Module<KittiesTest>;
type Balances = balances::Module<KittiesTest>;
type System = system::Module<KittiesTest>;
```

Finally, we create a wrapper function to create the previously mentioned `TestExternalities` for us.

```rust
fn build_ext() -> TestExternalities<Blake2Hasher> {
	let mut t = system::GenesisConfig::<KittiesTest>::default().build_storage().unwrap().0;
	t.extend(balances::GenesisConfig::<KittiesTest>::default().build_storage().unwrap().0);
	// t.extend(GenesisConfig::<KittiesTest>::default().build_ext().unwrap().0);
	t.into()
}
```

# Your Turn!

All is done! Except we don't have any tests. Try and add a single (and normal) rust test to the `tests` module. You know the drill, just add a `#[test]` compiler flag followed by a normal function. You can put an `assert!(true)` inside the test for now. Don't forget that the test must execute within the context of `TestExternalities`!


Solution:

```rust
#[test]
fn it_works() {
	with_externalities(&mut build_ext(), || {
		assert!(true);
	})
}
```

-------------- PART 2: writing some assertions using based on the module

The setup of tests might seem quite a lot, but it as worth noting that from this point onward, it is quite easy to interact with your module in an efficient way as much as you need. Furthermore, most of the steps above can be repeated for any module that you write, so you can come up with fancy ways to encapsulate the repetitive part and re-user it (curious? see `ExtBuilder` in some substrate runtime modules). Now, time for the easier stuff: playing around with the module.

There are many _point-of-views_ when it comes to writing tests. We are not really focused on that on that in the scope of this tutorial. Rather, we will focus on the act of writing them and technical details of it. Looking at the kitties module on a very rough scale, it does provide functionalities such as:

- Create a new Kitty
- Buy a Kitty.
- Send it to someone else.

Let's start by trying to test one of these. The code will look like this:

```rust
#[test]
fn create_kitty_should_work() {
	with_externalities(&mut build_ext(), || {
		// create a kitty with account 10.
		assert_ok!(Kitties::create_kitty(Origin::signed(10)));

		// check (some) storage items.
		assert_eq!(Kitties::all_kitties_count(), 1);
		assert_eq!(Kitties::owned_kitty_count(10), 1);
		// no one else has a kitty, only 10
		assert_eq!(Kitties::owned_kitty_count(5), 0);

		let hash = Kitties::kitty_by_index(0);
		assert_eq!(Kitties::owner_of(hash), Some(10));

		let other_hash = Kitties::kitty_of_owner_by_index((10, 0));
		assert_eq!(hash, other_hash);

		// alternative syntax:
		// TODO: explain this in a minimal manner without ::get().
		use super::KittyOwner;
		use support::StorageMap;
		assert_eq!(<KittyOwner<KittiesTest>>::get(hash), Some(10));
	})
}
```

Since you are already a good runtime developer, the above snippet should be quite clear to you! You also know how the kitties module works and why the assertions expect the aforementioned values. We will just point out some minor notes:

- `assert_ok!()` is a special macro that checks a dispatch call (remember that dispatch call return a special `Result<(), &'static str>`) and make sure that it is `Ok(())`. Furthermore, there is a `assert_noop!(function(), 'error_string')` that check that a call fails, whilst returning that particular error message (you might want to remember this for the end of this part).
- Notice how using a `u64` as `AccountId` is handy and simply `10` is the origin.
- Notice how we've used the same `Kitties` to call a getter function (e.g. `kitty_of_owner_by_index`) and a normal dispatch function (e.g. `create_kitty` from `decl_module!`).
- Note that you can use `Kitties` to call even the module functions as well (anything inside `impl Module<T>`)! Hence, you could (and should) have a test in which you only test the internal `mint()` function that is not directly accessible.

## Your Turn!

Even though this tutorial mentions that it does not want to teach you about _how to test software in general_ but rather focus on _how to write tests in substrate_, there are a few tips that might be useful.

- As mentioned, your dispatch calls should always start with checks and only if all the conditions are met, you start tweaking the state. You probably should have a test to make sure that each and every single one of those checks are working properly.
- TODO

So, to finish up this part:

- Write some a test to check
  - a successful transfer.
  - a failing transfer explicitly because of `ensure!(owner == from, "'from' account does not own this kitty");` (combined or separate).

- The solution will not contain more, but, of course, you are encouraged to write as many tests as you can at this point. Though, you might notice that for many tests you need some common setup (e.g. create 10 kitties for 10 account) and repeating them in each part is.. well not fun. Thankfully, __not repeating yourself__ is one of the main design goals of substrate and there is a way to get around it. We will get to this in the next section.

<!-- tabs:start -->

#### ** Solution **

[embedded-code-final](./assets/3.5-finished-code.rs ':include :type=code embed-final')

<!-- tabs:end -->