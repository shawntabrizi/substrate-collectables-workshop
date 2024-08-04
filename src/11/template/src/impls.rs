use super::*;
use frame::prelude::*;

impl<T: Config> Pallet<T> {
	pub fn mint(owner: T::AccountId) -> DispatchResult {
		let current_count: u64 = CountForKitties::<T>::get().unwrap_or(0);
		/* 🚧 TODO 🚧: Update this logic to use safe math. */
		let new_count = current_count + 1;
		CountForKitties::<T>::set(Some(new_count));
		Self::deposit_event(Event::<T>::Created { owner });
		Ok(())
	}
}
