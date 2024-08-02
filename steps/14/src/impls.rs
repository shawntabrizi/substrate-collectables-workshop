use super::*;
use frame_support::pallet_prelude::*;

impl<T: Config> Pallet<T> {
	pub fn mint(owner: T::AccountId) -> DispatchResult {
		/* TODO: Remove the `unwrap_or` which is not needed when using `ValueQuery`. */
		let current_count: u64 = CountForKitties::<T>::get().unwrap_or(0);
		let new_count = current_count.checked_add(1).ok_or(Error::<T>::TooManyKitties)?;
		/* TODO: Remove the `Option` wrapper when setting the `new_count`. */
		CountForKitties::<T>::set(Some(new_count));
		Self::deposit_event(Event::<T>::Created { owner });
		Ok(())
	}
}
