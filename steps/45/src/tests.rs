// Tests for the Kitties Pallet.
//
// Normally this file would be split into two parts: `mock.rs` and `tests.rs`.
// The `mock.rs` file would contain all the setup code for our `TestRuntime`.
// Then `tests.rs` would only have the tests for our pallet.
// However, to minimize the project, these have been combined into this single file.
//
// Learn more about creating tests for Pallets:
// https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html

// This flag tells rust to only run this file when running `cargo test`.
#![cfg(test)]

use crate as pallet_kitties;
use crate::*;
use frame::deps::sp_io;
use frame::runtime::prelude::*;
use frame::testing_prelude::*;
use frame::traits::fungible::*;

type Balance = u64;
type Block = frame_system::mocking::MockBlock<TestRuntime>;

construct_runtime! {
	pub struct TestRuntime {
		System: frame_system,
		Balances: pallet_balances,
		PalletKitties: pallet_kitties,
	}
}

const DEFAULT_KITTY: Kitty<TestRuntime> = Kitty { dna: [0u8; 32], owner: 1, price: None };
const ALICE: u64 = 1;
const BOB: u64 = 2;

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for TestRuntime {
	type Block = Block;
	type AccountData = pallet_balances::AccountData<Balance>;
}

#[derive_impl(pallet_balances::config_preludes::TestDefaultConfig)]
impl pallet_balances::Config for TestRuntime {
	type AccountStore = System;
	type Balance = Balance;
}

impl pallet_kitties::Config for TestRuntime {
	type RuntimeEvent = RuntimeEvent;
	type NativeBalance = Balances;
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::<TestRuntime>::default()
		.build_storage()
		.unwrap()
		.into()
}

#[test]
fn system_and_balances_work() {
	// This test will just sanity check that we can access `System` and `Balances`.
	new_test_ext().execute_with(|| {
		// We often need to set `System` to block 1 so that we can see events.
		System::set_block_number(1);
		// We often need to add some balance to a user to test features which needs tokens.
		assert_ok!(Balances::mint_into(&1, 100));
	});
}

#[test]
fn create_kitty_checks_signed() {
	new_test_ext().execute_with(|| {
		// The `create_kitty` extrinsic should work when being called by a user.
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(1)));
		// The `create_kitty` extrinsic should fail when being called by an unsigned message.
		assert_noop!(PalletKitties::create_kitty(RuntimeOrigin::none()), DispatchError::BadOrigin);
	})
}

#[test]
fn create_kitty_emits_event() {
	new_test_ext().execute_with(|| {
		// We need to set block number to 1 to view events.
		System::set_block_number(1);
		// Execute our call, and ensure it is successful.
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(1)));
		// Assert the last event by our blockchain is the `Created` event with the correct owner.
		System::assert_last_event(Event::<TestRuntime>::Created { owner: 1 }.into());
	})
}

#[test]
fn count_for_kitties_created_correctly() {
	new_test_ext().execute_with(|| {
		// Querying storage before anything is set will return `0`.
		assert_eq!(CountForKitties::<TestRuntime>::get(), 0);
		// You can `set` the value using an `u32`.
		CountForKitties::<TestRuntime>::set(1337u32);
		// You can `put` the value directly with a `u32`.
		CountForKitties::<TestRuntime>::put(1337u32);
	})
}

#[test]
fn mint_increments_count_for_kitty() {
	new_test_ext().execute_with(|| {
		// Querying storage before anything is set will return `0`.
		assert_eq!(CountForKitties::<TestRuntime>::get(), 0);
		// Call `mint` to create a new kitty.
		assert_ok!(PalletKitties::mint(1, [1u8; 32]));
		// Now the storage should be `1`
		assert_eq!(CountForKitties::<TestRuntime>::get(), 1);
		// Let's call it two more times...
		assert_ok!(PalletKitties::mint(2, [2u8; 32]));
		assert_ok!(PalletKitties::mint(3, [3u8; 32]));
		// Now the storage should be `3`
		assert_eq!(CountForKitties::<TestRuntime>::get(), 3);
	})
}

#[test]
fn mint_errors_when_overflow() {
	new_test_ext().execute_with(|| {
		// Set the count to the largest value possible.
		CountForKitties::<TestRuntime>::set(u32::MAX);
		// `create_kitty` should not succeed because of safe math.
		assert_noop!(
			PalletKitties::create_kitty(RuntimeOrigin::signed(1)),
			Error::<TestRuntime>::TooManyKitties
		);
	})
}

#[test]
fn kitties_map_created_correctly() {
	new_test_ext().execute_with(|| {
		let zero_key = [0u8; 32];
		assert_eq!(Kitties::<TestRuntime>::contains_key(zero_key), false);
		Kitties::<TestRuntime>::insert(zero_key, DEFAULT_KITTY);
		assert_eq!(Kitties::<TestRuntime>::contains_key(zero_key), true);
	})
}

#[test]
fn create_kitty_adds_to_map() {
	new_test_ext().execute_with(|| {
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(1)));
		assert_eq!(Kitties::<TestRuntime>::iter().count(), 1);
	})
}

#[test]
fn cannot_mint_duplicate_kitty() {
	new_test_ext().execute_with(|| {
		assert_ok!(PalletKitties::mint(1, [0u8; 32]));
		assert_noop!(PalletKitties::mint(2, [0u8; 32]), Error::<TestRuntime>::DuplicateKitty);
	})
}

#[test]
fn kitty_struct_has_expected_traits() {
	new_test_ext().execute_with(|| {
		let kitty = DEFAULT_KITTY;
		let bytes = kitty.encode();
		let _new_kitty = Kitty::<TestRuntime>::decode(&mut &bytes[..]).unwrap();
		assert!(Kitty::<TestRuntime>::max_encoded_len() > 0);
		let _info = Kitty::<TestRuntime>::type_info();
	})
}

#[test]
fn mint_stores_owner_in_kitty() {
	new_test_ext().execute_with(|| {
		assert_ok!(PalletKitties::mint(1337, [42u8; 32]));
		let kitty = Kitties::<TestRuntime>::get([42u8; 32]).unwrap();
		assert_eq!(kitty.owner, 1337);
		assert_eq!(kitty.dna, [42u8; 32]);
	})
}

#[test]
fn create_kitty_makes_unique_kitties() {
	new_test_ext().execute_with(|| {
		// Two calls to `create_kitty` should work.
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(1)));
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(2)));
		// And should result in two kitties in our system.
		assert_eq!(CountForKitties::<TestRuntime>::get(), 2);
		assert_eq!(Kitties::<TestRuntime>::iter().count(), 2);
	})
}

#[test]
fn kitties_owned_created_correctly() {
	new_test_ext().execute_with(|| {
		// Initially users have no kitties owned.
		assert_eq!(KittiesOwned::<TestRuntime>::get(1).len(), 0);
		// Let's create two kitties.
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(1)));
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(1)));
		// Now they should have two kitties owned.
		assert_eq!(KittiesOwned::<TestRuntime>::get(1).len(), 2);
	});
}

#[test]
fn cannot_own_too_many_kitties() {
	new_test_ext().execute_with(|| {
		// If your max owned is different than 100, you will need to update this.
		for _ in 0..100 {
			assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(1)));
		}
		assert_noop!(
			PalletKitties::create_kitty(RuntimeOrigin::signed(1)),
			Error::<TestRuntime>::TooManyOwned
		);
	});
}

#[test]
fn transfer_emits_event() {
	new_test_ext().execute_with(|| {
		// We need to set block number to 1 to view events.
		System::set_block_number(1);
		// Create a kitty to transfer
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(ALICE)));
		// Get the kitty id.
		let kitty_id = Kitties::<TestRuntime>::iter_keys().collect::<Vec<_>>()[0];
		assert_ok!(PalletKitties::transfer(RuntimeOrigin::signed(ALICE), BOB, kitty_id));
		System::assert_last_event(
			Event::<TestRuntime>::Transferred { from: ALICE, to: BOB, kitty_id }.into(),
		);
	});
}

#[test]
fn transfer_logic_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(ALICE)));
		// Starting state looks good.
		let kitty = &Kitties::<TestRuntime>::iter_values().collect::<Vec<_>>()[0];
		let kitty_id = kitty.dna;
		assert_eq!(kitty.owner, ALICE);
		assert_eq!(KittiesOwned::<TestRuntime>::get(ALICE), vec![kitty_id]);
		assert_eq!(KittiesOwned::<TestRuntime>::get(BOB), vec![]);
		// Cannot transfer to yourself.
		assert_noop!(
			PalletKitties::transfer(RuntimeOrigin::signed(ALICE), ALICE, kitty_id),
			Error::<TestRuntime>::TransferToSelf
		);
		// Cannot transfer a non-existent kitty.
		assert_noop!(
			PalletKitties::transfer(RuntimeOrigin::signed(ALICE), BOB, [0u8; 32]),
			Error::<TestRuntime>::NoKitty
		);
		// Cannot transfer kitty you do not own.
		assert_noop!(
			PalletKitties::transfer(RuntimeOrigin::signed(BOB), ALICE, kitty_id),
			Error::<TestRuntime>::NotOwner
		);
		// Transfer should work when parameters are right.
		assert_ok!(PalletKitties::transfer(RuntimeOrigin::signed(ALICE), BOB, kitty_id));
		// Storage is updated correctly.
		assert_eq!(KittiesOwned::<TestRuntime>::get(ALICE), vec![]);
		assert_eq!(KittiesOwned::<TestRuntime>::get(BOB), vec![kitty_id]);
		let kitty = &Kitties::<TestRuntime>::iter_values().collect::<Vec<_>>()[0];
		assert_eq!(kitty.owner, BOB);
	});
}

#[test]
fn native_balance_associated_type_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(<<TestRuntime as Config>::NativeBalance as Mutate<_>>::mint_into(&ALICE, 1337));
		assert_eq!(
			<<TestRuntime as Config>::NativeBalance as Inspect<_>>::total_balance(&ALICE),
			1337
		);
	});
}

#[test]
fn balance_of_type_works() {
	// Inside our tests, the `BalanceOf` type has a concrete type of `u64`.
	let _example_balance: BalanceOf<TestRuntime> = 1337u64;
}
