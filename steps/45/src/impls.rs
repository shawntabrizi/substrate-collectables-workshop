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
		let kitty = Kitty { dna, owner: owner.clone(), price: None };
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
		ensure!(from != to, Error::<T>::TransferToSelf);
		let mut kitty = Kitties::<T>::get(kitty_id).ok_or(Error::<T>::NoKitty)?;
		ensure!(kitty.owner == from, Error::<T>::NotOwner);
		kitty.owner = to.clone();

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

	pub fn do_set_price(
		caller: T::AccountId,
		kitty_id: [u8; 32],
		new_price: Option<BalanceOf<T>>,
	) -> DispatchResult {
		let mut kitty = Kitties::<T>::get(kitty_id).ok_or(Error::<T>::NoKitty)?;
		ensure!(kitty.owner == caller, Error::<T>::NotOwner);
		kitty.price = new_price;
		Kitties::<T>::insert(kitty_id, kitty);

		Self::deposit_event(Event::<T>::PriceSet { owner: caller, kitty_id, new_price });
		Ok(())
	}

	pub fn do_buy_kitty(
		buyer: T::AccountId,
		kitty_id: [u8; 32],
		price: BalanceOf<T>,
	) -> DispatchResult {
		/* 🚧 TODO 🚧: Sanity check that the purchase is allowed:
			- Get `kitty` from `Kitties` using `kitty_id`, `ok_or` return `Error::<T>::NoKitty`.
			- Get the `real_price` from `kitty.price`, `ok_or` return `Error::<T>::NotForSale`.
			- `ensure!` that `price` is greater or equal to `real_price`, else `Error::<T>::MaxPriceTooLow`.
		*/

		/* 🚧 TODO 🚧: Execute the transfers:
			- Import `use frame_support::traits::tokens::Preservation;`, which is used for balance transfer.
			- Use `T::NativeBalance` to `transfer` from the `buyer` to the `kitty.owner`.
				- The amount transferred should be the `real_price`.
				- Use `Preservation::Preserve` to ensure the buyer account stays alive.
			- Use `Self::do_transfer` to transfer from the `kitty.owner` to the `buyer` with `kitty_id`.
			- Remember to propagate up all results from these functions with `?`.
		*/

		/* 🚧 TODO 🚧: Update the event to use the `real_price` in the `Event`. */
		Self::deposit_event(Event::<T>::Sold { buyer, kitty_id, price });
		Ok(())
	}
}
