use super::*;
use frame_support::pallet_prelude::*;

/* TODO:  Learn about internal functions. */
impl<T: Config> Pallet<T> {
	pub fn mint(owner: T::AccountId) -> DispatchResult {
		Self::deposit_event(Event::<T>::Created { owner });
		Ok(())
	}
}
