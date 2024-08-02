use super::*;
use frame_support::pallet_prelude::*;

impl<T: Config> Pallet<T> {
	// Generates and returns DNA and Sex
	pub fn gen_dna() -> [u8; 16] {
		// Create randomness payload. Multiple kitties can be generated in the same block,
		// retaining uniqueness.
		let unique_payload = (
			frame_system::Pallet::<T>::parent_hash(),
			frame_system::Pallet::<T>::block_number(),
			frame_system::Pallet::<T>::extrinsic_index(),
			CountForKitties::<T>::get(),
		);

		let encoded_payload = unique_payload.encode();
		frame_support::Hashable::blake2_128(&encoded_payload)
	}

	pub fn mint(owner: T::AccountId, dna: [u8; 16]) -> DispatchResult {
		let kitty = Kitty { dna, owner: owner.clone() };
		// Check if the kitty does not already exist in our storage map
		ensure!(!Kitties::<T>::contains_key(dna), Error::<T>::DuplicateKitty);

		let current_count: u64 = CountForKitties::<T>::get();
		let new_count = current_count.checked_add(1).ok_or(Error::<T>::TooManyKitties)?;

		/* 🚧 TODO 🚧:
			- Update `append` to `try_append` and `map_err` to `Error::<T>::TooManyOwned`.
		*/
		KittiesOwned::<T>::append(&owner, dna);
		Kitties::<T>::insert(dna, kitty);
		CountForKitties::<T>::set(new_count);

		Self::deposit_event(Event::<T>::Created { owner });
		Ok(())
	}
}
