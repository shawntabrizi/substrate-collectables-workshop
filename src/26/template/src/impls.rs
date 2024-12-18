use super::*;
use frame::prelude::*;
use frame::primitives::BlakeTwo256;
use frame::traits::Hash;

// Learn about internal functions.
impl<T: Config> Pallet<T> {
	// Generates and returns DNA
	pub fn gen_dna() -> [u8; 32] {
		// Create randomness payload. Multiple kitties can be generated in the same block,
		// retaining uniqueness.
		let unique_payload = (
			frame_system::Pallet::<T>::parent_hash(),
			frame_system::Pallet::<T>::block_number(),
			frame_system::Pallet::<T>::extrinsic_index(),
			CountForKitties::<T>::get(),
		);

		BlakeTwo256::hash_of(&unique_payload).into()
	}

	pub fn mint(owner: T::AccountId, dna: [u8; 32]) -> DispatchResult {
		let kitty = Kitty { dna, owner: owner.clone() };
		// Check if the kitty does not already exist in our storage map
		ensure!(!Kitties::<T>::contains_key(dna), Error::<T>::DuplicateKitty);

		let current_count: u32 = CountForKitties::<T>::get();
		let new_count = current_count.checked_add(1).ok_or(Error::<T>::TooManyKitties)?;

		KittiesOwned::<T>::try_append(&owner, dna).map_err(|_| Error::<T>::TooManyOwned)?;
		Kitties::<T>::insert(dna, kitty);
		CountForKitties::<T>::set(new_count);

		Self::deposit_event(Event::<T>::Created { owner });
		Ok(())
	}

	/* 🚧 TODO 🚧: Create an internal function called `do_transfer`:
		- It has inputs:
			- `from` which is `T::AccountId`.
			- `to` which is `T::AccountId`.
			- `kitty_id` which is `[u8; 32]`.
		- It returns a `DispatchResult`
		- The inner logic for now is:
			- Call `Self::deposit_event` and emit `Event::<T>:Transferred` with params.
			- Return `Ok(())`.
	*/
}
