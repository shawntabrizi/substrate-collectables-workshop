use super::*;
use frame::prelude::*;
/* 🚧 TODO 🚧: Import `frame::primtives::BlakeTwo256`. */
/* 🚧 TODO 🚧: Import `frame::traits::Hash`. */

impl<T: Config> Pallet<T> {
	/* 🚧 TODO 🚧: Create a function `gen_dna` which returns a `[u8; 32]`.
		- Create a `unique_payload` which contains data from `frame_system::Pallet::<T>`:
			- `parent_hash`
			- `block_number`
			- `extrinsic_index`
			- `CountForKitties::<T>::get()`
		- Use `BlakeTwo256` to calculate the `hash_of` the unique payload.
		- Return the hash as a `[u8; 32]`.
	*/

	pub fn mint(owner: T::AccountId, dna: [u8; 32]) -> DispatchResult {
		let kitty = Kitty { dna, owner: owner.clone() };
		// Check if the kitty does not already exist in our storage map
		ensure!(!Kitties::<T>::contains_key(dna), Error::<T>::DuplicateKitty);

		let current_count: u64 = CountForKitties::<T>::get();
		let new_count = current_count.checked_add(1).ok_or(Error::<T>::TooManyKitties)?;
		Kitties::<T>::insert(dna, kitty);
		CountForKitties::<T>::set(new_count);
		Self::deposit_event(Event::<T>::Created { owner });
		Ok(())
	}
}
