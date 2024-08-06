# FRAME Macros

Rust allows you to write [macros](https://doc.rust-lang.org/book/ch19-06-macros.html), which is code that generates code.

FRAME uses Macros to simplify the development of Pallets, while keeping all of the benefits of using Rust.

You can identify most macros in one of two forms:

- `#[macro_name]`: Attribute macros, which are applied on top of valid rust syntax.
- `macro_name!(...)`: Declarative macros, which can define their own internal syntax.

## The Power of Macros

We can see a direct example of how much smaller we can make a Rust project by using macros to replace boilerplate code:

- `wc -l` will show the number of lines of a file.
- `cargo expand` will expand the macros to "pure" Rust.

```sh
➜  substrate git:(master) ✗ wc -l frame/sudo/src/lib.rs
    310 frame/sudo/src/lib.rs

➜  substrate git:(master) ✗ cargo expand -p pallet-sudo | wc -l
    2210
```

So this shows that a Pallet written with macros can be 7 times smaller than a Pallet which isn't.

## The Risk of Macros

One of the risks of using macros is the creation of "macro magic". This is slang for when macros do so much code generation, that the user is not even sure what is happening.

Especially with declarative macros, where users can basically create a new programming language within the macros.

The goal of FRAME macros is to stay as close to Rust as possible, but also remove all the boilerplate code that would otherwise be annoying to write.

We will call out such cases of macro magic in the next chapters.

## Macros in Our Template

Our starting template includes all the basic macros used for developing a FRAME pallet.

### Pallet Macro Entrypoint

The entrypoint for all the FRAME macros is can be seen here:

```rust
#[frame::pallet(dev_mode)]
pub mod pallet {
	// -- snip --
}
```

You will notice we wrap all of our Pallet code inside of this entrypoint, which allows our macros to have context of all the details inside.

More simply explained, if we had:

```rust
#[macro_1]
pub struct ItemOne;

#[macro_2]
pub struct ItemTwo;
```

There would be no way for `#[macro_1]` and `#[macro_2]` to communicate information to one another. However, with a design like:

```rust
#[macro_entrypoint]
pub mod pallet {
	#[macro_1]
	pub struct ItemOne;

	#[macro_2]
	pub struct ItemTwo;
}
```

We can now design the `#[macro_entrypoint]` to keep track of all data inside of the `mod pallet` container, and that means we can now design `#[macro_1]` and `#[macro_2]` to have context of one another, and interact with each other too.

The unfortunate limitation here is that wherever we want to use FRAME macros, we must basically do it in a single file and all enclosed by the `#[frame::pallet]` macro entrypoint.

We will go over each of the FRAME macros throughout this tutorial

### Basic Pallet Structure

While the template is already very minimal, you can mentally break it down like:

```rust
use frame::prelude::*;
pub use pallet::*;

#[frame::pallet]
pub mod pallet {
	use super::*;

	#[pallet::pallet]
	pub struct Pallet<T>(core::marker::PhantomData<T>);

	#[pallet::config]  // snip
	#[pallet::event]   // snip
	#[pallet::error]   // snip
	#[pallet::storage] // snip
	#[pallet::call]    // snip
}
```

Let's explore this further.
