Writing a Test
===


In addition, try to add a single passing rust test to your setup. You know the drill, just add a `#[test]` compiler flag followed by a normal function. You can put an `assert!(true)` inside the test for now. Don't forget that the test must execute within the context of `TestExternalities`!


Solution:

```rust
#[test]
fn it_works() {
	with_externalities(&mut build_ext(), || {
		assert!(true);
	})
}
```



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

<!-- [embedded-code-final](./assets/3.5-finished-code.rs ':include :type=code embed-final') -->

<!-- tabs:end -->