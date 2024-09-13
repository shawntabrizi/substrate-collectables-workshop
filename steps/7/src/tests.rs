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

// In our "test runtime", we represent a user `AccountId` with a `u64`.
// This is just a simplification so that we don't need to generate a bunch of proper cryptographic
// public keys when writing tests. It is just easier to say "user 1 transfers to user 2".
// We create the constants `ALICE` and `BOB` to make it clear when we are representing users below.
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
}

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
		assert_ok!(Balances::mint_into(&ALICE, 100));
		assert_ok!(Balances::mint_into(&BOB, 100));
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
