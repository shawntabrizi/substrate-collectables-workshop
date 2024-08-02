#![cfg_attr(not(feature = "std"), no_std)]

mod impls;

pub use pallet::*;

#[frame_support::pallet(dev_mode)]
pub mod pallet {
	use super::*;
	use frame_support::{
		pallet_prelude::*,
		traits::fungible::{Inspect, Mutate},
	};
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

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
		// Using 16 bytes to represent a kitty DNA
		pub dna: [u8; 16],
		pub owner: T::AccountId,
		pub price: Option<BalanceOf<T>>,
	}

	#[pallet::storage]
	pub(super) type CountForKitties<T: Config> = StorageValue<Value = u64, QueryKind = ValueQuery>;

	#[pallet::storage]
	pub(super) type Kitties<T: Config> = StorageMap<Key = [u8; 16], Value = Kitty<T>>;

	/// Track the kitties owned by each account.
	#[pallet::storage]
	pub(super) type KittiesOwned<T: Config> = StorageMap<
		Key = T::AccountId,
		Value = BoundedVec<[u8; 16], ConstU32<100>>,
		QueryKind = ValueQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Created { owner: T::AccountId },
		Transferred { from: T::AccountId, to: T::AccountId, kitty_id: [u8; 16] },
		PriceSet { owner: T::AccountId, kitty_id: [u8; 16], new_price: Option<BalanceOf<T>> },
		Sold { buyer: T::AccountId, kitty_id: [u8; 16], price: BalanceOf<T> },
	}

	#[pallet::error]
	pub enum Error<T> {
		TooManyKitties,
		DuplicateKitty,
		TooManyOwned,
		TransferToSelf,
		NoKitty,
		NotOwner,
		/* TODO: Add `Errors` needed for `do_buy_kitty`:
			- `NotForSale`: for when the Kitty has a price set to `None`.
			- `MaxPriceTooLow`: for when the price offered by the buyer is too low.
		*/
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
			kitty_id: [u8; 16],
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::do_transfer(who, to, kitty_id)?;
			Ok(())
		}

		pub fn set_price(
			origin: OriginFor<T>,
			kitty_id: [u8; 16],
			new_price: Option<BalanceOf<T>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::do_set_price(who, kitty_id, new_price)?;
			Ok(())
		}

		pub fn buy_kitty(
			origin: OriginFor<T>,
			kitty_id: [u8; 16],
			max_price: BalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::do_buy_kitty(who, kitty_id, max_price)?;
			Ok(())
		}
	}
}
