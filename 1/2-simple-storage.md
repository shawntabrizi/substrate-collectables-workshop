Simple Storage
===

Let's add the most simple logic we can to our runtime: a function which stores a variable.

To do this, we will first need to define a storage variable in the `decl_storage!` macro. This allows for type-safe usage of the Substrate storage database, so you can keep things around between blocks.

## Declaring a Storage Value

Substrate natively supports all the primitive types available in Rust (bool, u8, u32, etc..) and as well some custom types specific to Substrate (AccountId, Balance, Hash, and more...)

You can declare a simple storage item like this:

```
decl_storage! {
    trait Store for Module<T: Trait> as Example {
        MyU32: u32;
        MyBool get(my_bool_getter): bool;
    }
}
```

Here we have defined two variables: a u32 and a bool with a getter function. The `get` paramter is optional, but if you put add it to your storage item, it will expose a getter function with the name specified (`fn getter_name() -> Type`).

To store these basic storage values, you need to import the `support::StorageValue` module.

### Working with a Storage Value

The functions used to access a `StorageValue` is defined in the [`srml/support` folder](https://github.com/paritytech/substrate/blob/master/srml/support/src/storage/generator.rs#L98):

```
	/// Get the storage key.
	fn key() -> &'static [u8];

	/// true if the value is defined in storage.
	fn exists<S: Storage>(storage: &S) -> bool {
		storage.exists(Self::key())
	}

	/// Load the value from the provided storage instance.
	fn get<S: Storage>(storage: &S) -> Self::Query;

	/// Take a value from storage, removing it afterwards.
	fn take<S: Storage>(storage: &S) -> Self::Query;

	/// Store a value under this key into the provided storage instance.
	fn put<S: Storage>(val: &T, storage: &S) {
		storage.put(Self::key(), val)
	}

	/// Mutate this value
	fn mutate<R, F: FnOnce(&mut Self::Query) -> R, S: Storage>(f: F, storage: &S) -> R;

	/// Clear the storage value.
	fn kill<S: Storage>(storage: &S) {
		storage.kill(Self::key())
```

So if you want to "put" the value of `MyU32` you could write:

```
<MyU32<T>>::put(1337);
```

## Declaring a Public Function

We need to define runtime logic that will set and modify our storage values. To do that, we need to expose public entry points which can be called by signing and submitting an extrinsic.

Here is an example of a public function declaration:

```
decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        fn my_function(origin, input_bool: bool) -> Result {
            let _sender = ensure_signed(origin)?;

            <MyBool<T>>::put(input_bool);
            
            Ok(())
        }
    }
}
```

The first argument of these functions is always `origin`. This informs the runtime where the call originated from, whether it be a signed message (a transaction) or even an unsigned message (an inherent).

Additionally, these functions must return the `Result` type from the `srml_support::dispatch` module. This means that a successful function call will always return `Ok(())`, otherwise, the logic should catch any errors which may cause a problem and return an `Err()`.

Since these are dispatched functions, there are two extremely important things to remember:

 - MUST NOT PANIC: Under no circumstances (save, perhaps, storage getting into an irreparably damaged state) must this function panic.
 - NO SIDE-EFFECTS ON ERROR: This function must either complete totally and return `Ok(())`, or it must have no side-effects on storage and return `Err('Some reason')`.

Throughout this tutorial, we will make sure that both of these conditions are satisfied, and we will remind you to do the same.

## Checking for a Signed Message

As mentioned, the first argument in any of these public functions is the origin. We can use the `ensure_signed()` function from `system::ensure_signed` to check the origin, and "ensure" that the messaged is signed by a valid account. We can even derive the signing account from the result of the function call, but we won't be using that for this example (hence the underscore (`_`) telling the Rust compiler this variable is unused).

## Your Turn!
Use the template to create a `set_value()` function which will allow a user to send a signed message which puts a `u64` into the runtime storage.

Remember to import all the libraries required to get your code to compile successfully!

---
**Learn More**

If you try to compile the code samples we showed above without importing the required libraries, you will get some errors:

```
error[E0425]: cannot find function `ensure_signed` in this scope
  --> src/cryptokitties.rs:15:27
   |
15 |             let _sender = ensure_signed(origin)?;
   |                           ^^^^^^^^^^^^^ not found in this scope
help: possible candidate is found in another module, you can import it into scope
   |
2  | use system::ensure_signed;
   |

error[E0599]: no function or associated item named `put` found for type `cryptokitties::Value<T>` in the current scope
  --> src/cryptokitties.rs:16:25
   |
6  | decl_storage! {
   | ------------- function or associated item `put` not found for this
...
16 |             <MyU32<T>>::put(input);
   |             ------------^^^
   |             |
   |             function or associated item not found in `cryptokitties::MyU32<T>`
   |
   = help: items from traits can only be used if the trait is in scope
   = note: the following traits are implemented but not in scope, perhaps add a `use` for one of them:
           candidate #1: `use srml_support::Storage;`
           candidate #2: `use srml_support::storage::generator::StorageValue;`
           candidate #3: `use srml_support::StorageValue;`

...
```

As you can see, we added use some functionality in our code that we had not yet imported into our module. Rust actually can help us here by suggesting ways to solve these problems. If we listen to Rust, then we can simply add these `use` statements at the very top to get our code to compile again:

```
use system::ensure_signed;
use srml_support::{StorageValue, dispatch::Result};
```

As mentioned in "common patterns" section, Rust will be your friend throughout runtime development, and *should mostly* help you overcome any issues in your code. Moving forward we will try to mention whenever you need to import a new library, but don't fear when the compiler sends you some errors. Instead embrace the helpful suggestions it might be giving you.

---