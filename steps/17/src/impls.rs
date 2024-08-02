use super::*;
use frame_support::pallet_prelude::*;

impl<T: Config> Pallet<T> {
	/* 🚧 TODO 🚧: Update this function signature to include `id` which is type `[u8; 16]`. */
	pub fn mint(owner: T::AccountId) -> DispatchResult {
		let current_count: u64 = CountForKitties::<T>::get();
		let new_count = current_count.checked_add(1).ok_or(Error::<T>::TooManyKitties)?;
		/* 🚧 TODO 🚧: In the `Kitties` map, under the key `id`, insert `()`. */
		CountForKitties::<T>::set(new_count);
		Self::deposit_event(Event::<T>::Created { owner });
		Ok(())
	}
}
