use super::*;
use frame::prelude::*;

impl<T: Config> Pallet<T> {
	pub fn mint(owner: T::AccountId) -> DispatchResult {
		Self::deposit_event(Event::<T>::Created { owner });
		Ok(())
	}
}
