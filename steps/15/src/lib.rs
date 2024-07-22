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

	/// Learn about storage value.
	#[pallet::storage]
	pub(super) type CountForKitties<T: Config> = StorageValue<Value = u64>;

	/// Learn about storage maps.
	#[pallet::storage]
	pub(super) type Kitties<T: Config> = StorageMap<Key = [u8; 16], Value = ()>;

	// Learn about events.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Created { owner: T::AccountId },
	}

	#[pallet::error]
	pub enum Error<T> {
		TooManyKitties,
	}

	// Learn about callable functions and dispatch.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		pub fn create_kitty(origin: OriginFor<T>) -> DispatchResult {
			// Learn about `origin`.
			let who = ensure_signed(origin)?;
			/* TODO: Create `const default_id`, which type `[u8; 16]` and has value `[0u8; 16]`. */
			/* TODO: Pass `default_id` to the `mint` function. */
			Self::mint(who)?;
			Ok(())
		}
	}

	// Learn about internal functions.
	impl<T: Config> Pallet<T> {
		// Learn about `AccountId`.
		/* TODO: Update this function signature to include `id` which is type `[u8; 16]`. */
		fn mint(owner: T::AccountId) -> DispatchResult {
			let current_count: u64 = CountForKitties::<T>::get().unwrap_or(0);
			let new_count = current_count.checked_add(1).ok_or(Error::<T>::TooManyKitties)?;
			/* TODO: In the `Kitties` map, under the key `id`, insert `()`. */
			CountForKitties::<T>::set(Some(new_count));
			Self::deposit_event(Event::<T>::Created { owner });
			Ok(())
		}
	}
}
