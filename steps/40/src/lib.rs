#![cfg_attr(not(feature = "std"), no_std)]

mod impls;

use frame::prelude::*;
pub use pallet::*;

#[frame::pallet(dev_mode)]
pub mod pallet {
	use super::*;
	use frame::traits::fungible::{Inspect, Mutate};

	#[pallet::pallet]
	pub struct Pallet<T>(core::marker::PhantomData<T>);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// The Fungible handler for the kitties pallet.
		type NativeBalance: Inspect<Self::AccountId> + Mutate<Self::AccountId>;
	}

	// Allows easy access our Pallet's `Balance` type. Comes from `Fungible` interface.
	pub type BalanceOf<T> =
		<<T as Config>::NativeBalance as Inspect<<T as frame_system::Config>::AccountId>>::Balance;

	#[derive(Encode, Decode, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct Kitty<T: Config> {
		// Using 32 bytes to represent a kitty DNA
		pub dna: [u8; 32],
		pub owner: T::AccountId,
		pub price: Option<BalanceOf<T>>,
	}

	#[pallet::storage]
	pub(super) type CountForKitties<T: Config> = StorageValue<Value = u64, QueryKind = ValueQuery>;

	#[pallet::storage]
	pub(super) type Kitties<T: Config> = StorageMap<Key = [u8; 32], Value = Kitty<T>>;

	/// Track the kitties owned by each account.
	#[pallet::storage]
	pub(super) type KittiesOwned<T: Config> = StorageMap<
		Key = T::AccountId,
		Value = BoundedVec<[u8; 32], ConstU32<100>>,
		QueryKind = ValueQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Created { owner: T::AccountId },
		Transferred { from: T::AccountId, to: T::AccountId, kitty_id: [u8; 32] },
		/* 🚧 TODO 🚧: Create a new `Event` called `PriceSet` with fields:
			- `owner` which is `T::AccountId`.
			- `kitty_id` which is `[u8; 32]`.
			- `new_price` which is `Option<BalanceOf<T>>`.
		*/
	}

	#[pallet::error]
	pub enum Error<T> {
		TooManyKitties,
		DuplicateKitty,
		TooManyOwned,
		TransferToSelf,
		NoKitty,
		NotOwner,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		pub fn create_kitty(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let dna = Self::gen_dna();
			Self::mint(who, dna)?;
			Ok(())
		}

		pub fn transfer(
			origin: OriginFor<T>,
			to: T::AccountId,
			kitty_id: [u8; 32],
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::do_transfer(who, to, kitty_id)?;
			Ok(())
		}

		/* 🚧 TODO 🚧: Make an callable function called `set_price`:
			- Inputs to the function are:
				- `origin` which is `OriginFor<T>`.
				- `kitty_id` which is `[u8; 32]`.
				- `new_price` which is `Option<BalanceOf<T>`.
			- Returns a `DispatchResult`
			- The internal logic, for now, should be:
				- Extract the caller `who` with `ensure_signed`.
				- Call `Self::do_set_price` with the appropriate parameters, propagating the result.
				- Return `Ok(())`.
		*/
	}
}
