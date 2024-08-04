#![cfg_attr(not(feature = "std"), no_std)]

mod impls;

use frame::prelude::*;
pub use pallet::*;

#[frame::pallet(dev_mode)]
pub mod pallet {
	use super::*;

	#[pallet::pallet]
	pub struct Pallet<T>(core::marker::PhantomData<T>);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[derive(Encode, Decode, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct Kitty<T: Config> {
		// Using 32 bytes to represent a kitty DNA
		pub dna: [u8; 32],
		pub owner: T::AccountId,
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
		/* 🚧 TODO 🚧: Create a new event called `Transferred`:
			- Parameters are:
				- `from` which is `T::AccountId`.
				- `to` which is `T::AccountId`.
				- `kitty_id` which is `[u8; 32]`.
		*/
	}

	#[pallet::error]
	pub enum Error<T> {
		TooManyKitties,
		DuplicateKitty,
		TooManyOwned,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		pub fn create_kitty(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let dna = Self::gen_dna();
			Self::mint(who, dna)?;
			Ok(())
		}

		/* 🚧 TODO 🚧: Make a new extrinsic called `transfer`.
			- Input parameters are:
				- `origin` which is `OriginFor<T>`.
				- `to` which is `T::AccountId`.
				- `kitty_id` which is `[u8; 32]`.
			- Returns a `DispatchResult`.
			- The inner logic should be:
				- Get the caller `who` from `ensure_signed`.
				- Call `Self::do_transfer`, and propagate the result.
				- End with Ok(()).
		*/
	}
}
