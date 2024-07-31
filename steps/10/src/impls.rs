use super::*;
use frame_support::pallet_prelude::*;

impl<T: Config> Pallet<T> {
	// Learn about `AccountId`.
	pub fn mint(owner: T::AccountId) -> DispatchResult {
		/* TODO:
			- `get` the current count of kitties.
			- `unwrap_or` set the count to `0`.
			- increment the count by one.
			- `set` the new count of kitties.
		*/
		Self::deposit_event(Event::<T>::Created { owner });
		Ok(())
	}
}
