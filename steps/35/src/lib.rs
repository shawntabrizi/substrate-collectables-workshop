#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

// Learn about Macros used in the `polkadot-sdk`, making pallet development easier.
#[frame_support::pallet(dev_mode)]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	// Learn about the Pallet struct: the structure on which we implement all functions and traits
	// for the Pallet.
	#[pallet::pallet]
	pub struct Pallet<T>(_);

	// Learn about frame_system, and `Config`.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[derive(Encode, Decode, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct Kitty<T: Config> {
		// Using 16 bytes to represent a kitty DNA
		pub dna: [u8; 16],
		pub owner: T::AccountId,
	}

	/// Learn about storage value.
	#[pallet::storage]
	pub(super) type CountForKitties<T: Config> = StorageValue<Value = u64, QueryKind = ValueQuery>;

	/// Learn about storage maps.
	#[pallet::storage]
	pub(super) type Kitties<T: Config> = StorageMap<Key = [u8; 16], Value = Kitty<T>>;

	/// Track the kitties owned by each account.
	#[pallet::storage]
	pub(super) type KittiesOwned<T: Config> = StorageMap<
		Key = T::AccountId,
		Value = BoundedVec<[u8; 16], ConstU32<100>>,
		QueryKind = ValueQuery,
	>;

	// Learn about events.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Created { owner: T::AccountId },
		Transferred { from: T::AccountId, to: T::AccountId, kitty_id: [u8; 16] },
	}

	#[pallet::error]
	pub enum Error<T> {
		TooManyKitties,
		DuplicateKitty,
		TooManyOwned,
		/* Add new `Error` variants needed for `do_transfer`:
			- `TransferToSelf`: for when the `from` and `to` of the transfer is the same.
			- `NoKitty`: for when a transfer involves a kitty that does not exist.
			- `NotOwner`: for when a transfer is initiated by someone who is not the current owner.
		*/
	}

	// Learn about callable functions and dispatch.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		pub fn create_kitty(origin: OriginFor<T>) -> DispatchResult {
			// Learn about `origin`.
			let who = ensure_signed(origin)?;
			let dna = Self::gen_dna();
			Self::mint(who, dna)?;
			Ok(())
		}

		pub fn transfer(
			origin: OriginFor<T>,
			to: T::AccountId,
			kitty_id: [u8; 16],
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::do_transfer(who, to, kitty_id)?;
			Ok(())
		}
	}

	// Learn about internal functions.
	impl<T: Config> Pallet<T> {
		// Generates and returns DNA and Sex
		fn gen_dna() -> [u8; 16] {
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

		// Learn about `AccountId`.
		pub fn mint(owner: T::AccountId, dna: [u8; 16]) -> DispatchResult {
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

		// Update storage to transfer kitty
		pub fn do_transfer(
			from: T::AccountId,
			to: T::AccountId,
			kitty_id: [u8; 16],
		) -> DispatchResult {
			/* TODO: Sanity check the transfer is allowed:
				- First `ensure!` that `from` and `to` are not equal, else return `Error::<T>::TransferToSelf`.
				- Get the `kitty` from `Kitties` using `kitty_id`, else return `Error::<T>::NoKitty`.
				- Check the `kitty.owner` is equal to `from`, else return `NotOwner`.
			*/

			/* TODO: Update the owner of the kitty:
				- Update `kitty.owner` to `to`.
				 - Update the `KittiesOwned` of `from` and `to:
					- Create a mutable `to_owned` by querying `KittiesOwned` for `to`.
					- `try_push` the `kitty_id` to the `to_owned` vector.
						- If the vector is full, `map_err` and return `Error::<T>::TooManyOwned`.
					- Create a mutable `from_owned` by querying `KittiesOwned` for `from`.
					- Write logic to `swap_remove` the item from the `from_owned` vector.
						- If you cannot find the kitty in the vector, return `Error::<T>::NoKitty`.
			*/

			/* TODO: Update the final storage.
				- Insert into `Kitties` under `kitty_id` the modified `kitty` struct.
				- Insert into `KittiesOwned` under `to` the modified `to_owned` vector.
				- Insert into `KittiesOwned` under `from` the modified `from_owned` vector.
			*/

			Self::deposit_event(Event::<T>::Transferred { from, to, kitty_id });
			Ok(())
		}
	}
}
