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
		let kitty = Kitty { dna, owner: owner.clone(), price: None };
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

	pub fn do_transfer(from: T::AccountId, to: T::AccountId, kitty_id: [u8; 32]) -> DispatchResult {
		ensure!(from != to, Error::<T>::TransferToSelf);
		let mut kitty = Kitties::<T>::get(kitty_id).ok_or(Error::<T>::NoKitty)?;
		ensure!(kitty.owner == from, Error::<T>::NotOwner);
		kitty.owner = to.clone();
		kitty.price = None;

		let mut to_owned = KittiesOwned::<T>::get(&to);
		to_owned.try_push(kitty_id).map_err(|_| Error::<T>::TooManyOwned)?;
		let mut from_owned = KittiesOwned::<T>::get(&from);
		if let Some(ind) = from_owned.iter().position(|&id| id == kitty_id) {
			from_owned.swap_remove(ind);
		} else {
			return Err(Error::<T>::NoKitty.into())
		}

		Kitties::<T>::insert(kitty_id, kitty);
		KittiesOwned::<T>::insert(&to, to_owned);
		KittiesOwned::<T>::insert(&from, from_owned);

		Self::deposit_event(Event::<T>::Transferred { from, to, kitty_id });
		Ok(())
	}

	/* 🚧 TODO 🚧: Make an internal function called `do_set_price`:
		- Inputs to the function are:
			- `caller` which is `T::AccountId`.
			- `kitty_id` which is `[u8; 32]`.
			- `new_price` which is `Option<BalanceOf<T>`.
		- Returns a `DispatchResult`.
		- The internal logic, for now, should be:
			- `Self::deposit_event` for `Event::<T>::PriceSet` with the appropriate params.
			- Return `Ok(())`.
	*/
}
