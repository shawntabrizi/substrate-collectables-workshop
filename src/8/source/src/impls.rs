use super::*;
use frame_support::pallet_prelude::*;

impl<T: Config> Pallet<T> {
	pub fn mint(owner: T::AccountId) -> DispatchResult {
		Self::deposit_event(Event::<T>::Created { owner });
		Ok(())
	}
}
