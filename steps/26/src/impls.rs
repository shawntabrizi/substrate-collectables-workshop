use super::*;
use frame_support::pallet_prelude::*;

impl<T: Config> Pallet<T> {
	/* TODO: Create a function `gen_dna` which returns a `[u8; 16]`.
		- Create a `unique_payload` which contains data from `frame_system::Pallet::<T>`:
			- `parent_hash`
			- `block_number`
			- `extrinsic_index`
			- `CountForKitties::<T>::get()`
		- `encode()` that payload to a byte array named `encoded_payload`.
		- Use `frame_support::Hashable` to perform a `blake2_128` hash on the encoded payload.
		- Return the 16 byte hash.
	*/

	pub fn mint(owner: T::AccountId, dna: [u8; 16]) -> DispatchResult {
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
