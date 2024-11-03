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

// In our "test runtime", we represent a user `AccountId` with a `u64`.
// This is just a simplification so that we don't need to generate a bunch of proper cryptographic
// public keys when writing tests. It is just easier to say "user 1 transfers to user 2".
// We create the constants `ALICE` and `BOB` to make it clear when we are representing users below.
const ALICE: u64 = 1;
const BOB: u64 = 2;

// Our blockchain tests only need 3 Pallets:
// 1. System: Which is included with every FRAME runtime.
// 2. PalletBalances: Which is manages your blockchain's native currency. (i.e. DOT on Polkadot)
// 3. PalletKitties: The pallet you are building in this tutorial!
construct_runtime! {
	pub struct TestRuntime {
		System: frame_system,
		PalletBalances: pallet_balances,
		PalletKitties: pallet_kitties,
	}
}

// Normally `System` would have many more configurations, but you can see that we use some macro
// magic to automatically configure most of the pallet for a "default test configuration".
#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for TestRuntime {
	type Block = Block;
	type AccountData = pallet_balances::AccountData<Balance>;
}

// Normally `Balances` would have many more configurations, but you can see that we use some macro
// magic to automatically configure most of the pallet for a "default test configuration".
#[derive_impl(pallet_balances::config_preludes::TestDefaultConfig)]
impl pallet_balances::Config for TestRuntime {
	type AccountStore = System;
	type Balance = Balance;
}

// This is the configuration of our Pallet! If you make changes to the pallet's `trait Config`, you
// will also need to update this configuration to represent that.
impl pallet_kitties::Config for TestRuntime {
	type RuntimeEvent = RuntimeEvent;
}

// We need to run most of our tests using this function: `new_test_ext().execute_with(|| { ... });`
// It simulates the blockchain database backend for our tests.
// If you forget to include this and try to access your Pallet storage, you will get an error like:
// "`get_version_1` called outside of an Externalities-provided environment."
pub fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::<TestRuntime>::default()
		.build_storage()
		.unwrap()
		.into()
}

#[test]
fn starting_template_is_sane() {
	new_test_ext().execute_with(|| {
		let event = Event::<TestRuntime>::Created { owner: ALICE };
		let _runtime_event: RuntimeEvent = event.into();
		let _call = Call::<TestRuntime>::create_kitty {};
		let result = PalletKitties::create_kitty(RuntimeOrigin::signed(BOB));
		assert_ok!(result);
	});
}

#[test]
fn system_and_balances_work() {
	// This test will just sanity check that we can access `System` and `Balances`.
	new_test_ext().execute_with(|| {
		// We often need to set `System` to block 1 so that we can see events.
		System::set_block_number(1);
		// We often need to add some balance to a user to test features which needs tokens.
		assert_ok!(PalletBalances::mint_into(&ALICE, 100));
		assert_ok!(PalletBalances::mint_into(&BOB, 100));
	});
}

#[test]
fn create_kitty_checks_signed() {
	new_test_ext().execute_with(|| {
		// The `create_kitty` extrinsic should work when being called by a user.
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(ALICE)));
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
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(ALICE)));
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
		// Call `create_kitty` which will call `mint`.
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(ALICE)));
		// Now the storage should be `1`
		assert_eq!(CountForKitties::<TestRuntime>::get(), 1);
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
		assert!(!Kitties::<TestRuntime>::contains_key(zero_key));
		Kitties::<TestRuntime>::insert(zero_key, ());
		assert!(Kitties::<TestRuntime>::contains_key(zero_key));
	})
}
