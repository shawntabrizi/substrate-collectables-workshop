use super::*;
use frame_support::pallet_prelude::*;

// Learn about internal functions.
impl<T: Config> Pallet<T> {
	// Generates and returns DNA and Sex
	pub fn gen_dna() -> [u8; 32] {
		// Create randomness payload. Multiple kitties can be generated in the same block,
		// retaining uniqueness.
		let unique_payload = (
			frame_system::Pallet::<T>::parent_hash(),
			frame_system::Pallet::<T>::block_number(),
			frame_system::Pallet::<T>::extrinsic_index(),
			CountForKitties::<T>::get(),
		);

		let encoded_payload = unique_payload.encode();
		frame_support::Hashable::blake2_256(&encoded_payload)
	}

	pub fn mint(owner: T::AccountId, dna: [u8; 32]) -> DispatchResult {
		let kitty = Kitty { dna, owner: owner.clone() };
		// Check if the kitty does not already exist in our storage map
		ensure!(!Kitties::<T>::contains_key(dna), Error::<T>::DuplicateKitty);

		let current_count: u64 = CountForKitties::<T>::get();
		let new_count = current_count.checked_add(1).ok_or(Error::<T>::TooManyKitties)?;

		KittiesOwned::<T>::try_append(&owner, dna).map_err(|_| Error::<T>::TooManyOwned)?;
		Kitties::<T>::insert(dna, kitty);
		CountForKitties::<T>::set(new_count);

		Self::deposit_event(Event::<T>::Created { owner });
		Ok(())
	}

	pub fn do_transfer(from: T::AccountId, to: T::AccountId, kitty_id: [u8; 32]) -> DispatchResult {
		/* 🚧 TODO 🚧: Sanity check the transfer is allowed:
			- First `ensure!` that `from` and `to` are not equal, else return `Error::<T>::TransferToSelf`.
			- Get the `kitty` from `Kitties` using `kitty_id`, else return `Error::<T>::NoKitty`.
			- Check the `kitty.owner` is equal to `from`, else return `NotOwner`.
		*/

		/* 🚧 TODO 🚧: Update the owner of the kitty:
			- Update `kitty.owner` to `to`.
			 - Update the `KittiesOwned` of `from` and `to:
				- Create a mutable `to_owned` by querying `KittiesOwned` for `to`.
				- `try_push` the `kitty_id` to the `to_owned` vector.
					- If the vector is full, `map_err` and return `Error::<T>::TooManyOwned`.
				- Create a mutable `from_owned` by querying `KittiesOwned` for `from`.
				- Write logic to `swap_remove` the item from the `from_owned` vector.
					- If you cannot find the kitty in the vector, return `Error::<T>::NoKitty`.
		*/

		/* 🚧 TODO 🚧: Update the final storage.
			- Insert into `Kitties` under `kitty_id` the modified `kitty` struct.
			- Insert into `KittiesOwned` under `to` the modified `to_owned` vector.
			- Insert into `KittiesOwned` under `from` the modified `from_owned` vector.
		*/

		Self::deposit_event(Event::<T>::Transferred { from, to, kitty_id });
		Ok(())
	}
}
