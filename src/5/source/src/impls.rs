use super::*;
use frame::prelude::*;

impl<T: Config> Pallet<T> {
	/* 🚧 TODO 🚧: Learn about `AccountId`. */
	pub fn mint(owner: T::AccountId) -> DispatchResult {
		Self::deposit_event(Event::<T>::Created { owner });
		Ok(())
	}
}
