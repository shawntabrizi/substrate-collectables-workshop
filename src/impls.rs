use super::*;
use frame::prelude::*;

impl<T: Config> Pallet<T> {
	pub fn mint(owner: T::AccountId) -> DispatchResult {
		/* 🚧 TODO 🚧:
			- `get` the `current_count` of kitties.
			- `unwrap_or` set the count to `0`.
			- Create `new_count` by adding one to the `current_count`.
			- `set` the `new_count` of kitties.
		*/
		Self::deposit_event(Event::<T>::Created { owner });
		Ok(())
	}
}
