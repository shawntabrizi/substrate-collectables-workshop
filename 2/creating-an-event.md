Creating an Event
===

On Substrate, [**Transactions**](https://substrate.dev/docs/en/overview/glossary#transaction) are handled differently than you might have been used to on Ethereum. Even though a transaction may be finalized, it does not necessarily imply that the function executed by that transaction fully succeeded.

To know that, we should emit an [**`Event`**](https://substrate.dev/docs/en/overview/glossary#events) at the end of the function to not only report success, but to tell the "off-chain world" that some particular state transition has happened.

## Declaring an Event

Substrate provides a `decl_event!` macro which allows you to easily create events which you can deposit in your runtime logic.

Here is an example of an event declaration:

```rust
decl_event!(
    pub enum Event<T>
    where
        <T as system::Trait>::AccountId,
        <T as system::Trait>::Balance
    {
        MyEvent(u32, Balance),
        MyOtherEvent(Balance, AccountId),
    }
);
```

Again, if we want to use some custom Substrate types, we need to integrate generics into our event definition.

## Adding an Event Type

The `decl_event!` macro will generate a new `Event` type which you will need to expose in your module. This type will need to inherit some traits like so:

```rust
pub trait Trait: balances::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}
```

## Depositing an Event

In order to use events within your runtime, you need to add a function which deposits those events. Since this is a common pattern in runtime development, the [**`decl_module!`**](https://github.com/paritytech/wiki/pull/272) macro can automatically add a default implementation of this to your module.

Simply add a new function to your module like so:

```rust
fn deposit_event<T>() = default;
```

If your events do not use any generics (e.g. just Rust primitive types), you should omit the `<T>` like this:

```rust
fn deposit_event() = default;
```

The `decl_module!` macro will detect this line, and replace it with the appropriate function definition for your runtime.

### Calling `deposit_event()`

Now that you have things set up within your module, you may want to actually deposit the event at the end of your function.

Doing that is relatively simple, you just need to provide the values that go along with your `Event` definition:

```rust
let my_value = 1337;
let my_balance = <T::Balance as As<u64>>::sa(1337);

Self::deposit_event(RawEvent::MyEvent(my_value, my_balance));
```

## Updating `lib.rs` to Include Events

The last step to get your runtime to compile is to update the `lib.rs` file to include the new `Event` type you have defined.

In your modules `Trait` implementation, you need to include:

```rust
// `lib.rs`
...
impl mymodule::Trait for Runtime {
    type Event = Event;
}
...
```

Finally, you need to also include the `Event` or `Event<T>` type to your module's definition in the `construct_runtime!` macro. Which one you include depends again on whether your event uses any generics.

```rust
// `lib.rs`
...
construct_runtime!(
    pub enum Runtime with Log(InternalLog: DigestItem<Hash, Ed25519AuthorityId>) where
        Block = Block,
        NodeBlock = opaque::Block,
        InherentData = BasicInherentData
    {
        ...
        MyModule: mymodule::{Module, Call, Storage, Event<T>},
    }
);
...
```

## Your Turn!

You will need to update your module to support events. We have taught you all the different parts, you will just need to piece it together.

Use the instructions in the template of this section to help you get your module talking! Remember you will need to update `lib.rs` too!

<!-- tabs:start -->

#### ** Template **

[embedded-code-template](./assets/2.2-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](./assets/2.2-finished-code.rs ':include :type=code embed-final')

#### ** Previous Chapter Solution **

[embedded-code-previous](./assets/2.1-finished-code.rs ':include :type=code embed-previous')

<!-- tabs:end -->
