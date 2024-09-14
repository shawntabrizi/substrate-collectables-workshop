# Pallet Events

The last thing we have included in our starting template is a simple event.

When a callable function completes successfully, there is often some metadata you would like to expose to the outside world about what exactly happened during the execution.

Events allow Pallets to express that something has happened, and allows off-chain systems like indexers or block explorers to track certain state transitions.

## Event Macro

The `#[pallet::event]` macro acts on an `enum Event`.

```rust
#[pallet::event]
#[pallet::generate_deposit(pub(super) fn deposit_event)]
pub enum Event<T: Config> {
	Created { owner: T::AccountId },
}
```

In this enum, you can introduce new variants as objects with arbitrary fields. Obviously you don't want to stick a ton of data in there, but you should feel comfortable to put the data which is relevant for tools like indexers and block explorers.

In this case, we set you up with a simple event stating a new kitty was created, and by whom. Of course there is no logic which is actually doing that yet, but that is what we will start to work on next.

Emitting an event is usually the last thing you will do in your extrinsic, noting when everything is done and with any final values you might have generated.

We will probably want to update our `Created` event with details about the Kitty we created. We can do that in the future.

## Macro Magic

You might ask, "What is this `generate_deposit` stuff?

When we deposit an event, we actually have to pass our event to `frame_system`, which manages events across all pallets.

The code for that function is:

```rust
impl<T: Config> Pallet<T> {
	pub(super) fn deposit_event(event: Event<T>) {
		let event = <<T as Config>::RuntimeEvent as From<Event<T>>>::from(event);
		let event = <<T as Config>::RuntimeEvent as Into<
				<T as frame_system::Config>::RuntimeEvent,
			>>::into(event);
		<frame_system::Pallet<T>>::deposit_event(event)
	}
}
```

Rather than asking the user to remember and write this every time, we are able to automatically generate it for the user.

Do you not like macro magic?

Delete the `generate_deposit` line, and copy and paste this code block into your code!

It is literally the same. In this case, I think the macro magic is justified.

You are able to access this function like you could any other function implemented on `Pallet`:

```rust
Self::deposit_event(Event::<T>::Created { owner });
```

As you see in our starting code.

## Tests

Don't forget to update your `tests.rs` file to include the test provided in this step.

It shows how you can:

- Set the blocknumber of your blockchain inside your tests.
- Call an extrinsic in your pallet from an `AccountId` of your choice.
- Check the extrinsic call completed `Ok(())`.
- Get the last event deposited into `System`.
- Check that last event matches the event you would expect from your pallet.

From this point forward, every step where you write some code will include new tests or modify existing tests.
Make sure to keep updating your `tests.rs` file throughout the tutorial.
