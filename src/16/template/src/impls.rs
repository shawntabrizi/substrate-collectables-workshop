use super::*;
use frame::prelude::*;

impl<T: Config> Pallet<T> {
	/* 🚧 TODO 🚧: Update this function signature to include `dna` which is type `[u8; 32]`. */
	pub fn mint(owner: T::AccountId) -> DispatchResult {
		let current_count: u32 = CountForKitties::<T>::get();
		let new_count = current_count.checked_add(1).ok_or(Error::<T>::TooManyKitties)?;
		/* 🚧 TODO 🚧: In the `Kitties` map, under the key `dna`, insert `()`. */
		CountForKitties::<T>::set(new_count);
		Self::deposit_event(Event::<T>::Created { owner });
		Ok(())
	}
}
