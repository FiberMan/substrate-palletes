//! Reputation Pallet
#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;
use sp_runtime::traits::{StaticLookup, Zero};
// use sp_std::prelude::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	// Define the pallet struct placeholder, various pallet function are implemented on it.
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// For constraining the maximum reputation for a call
		#[pallet::constant]
		type MaxReputationChange: Get<u32>;
	}

	// Pallets use events to inform users when important changes are made.
	// Event documentation should end with an array that provides descriptive names for parameters.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event emitted when reputation increased. [by who, for who, reputation]
		ReputationIncreased(T::AccountId, T::AccountId, u32),
		/// Event emitted when reputation decreased. [by who, for who, reputation]
		ReputationDecreased(T::AccountId, T::AccountId, u32),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Reputation is too low.
		ReputationTooLow,
		/// Reputation is too high.
		ReputationTooHigh,
		/// Account doesn't have reputation to decrease.
		NoReputationToDecrease,
	}

	#[pallet::storage]
	/// Maps each account to its reputation
	pub(super) type Reputations<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		u32,
	>;

	// Dispatchable functions allow users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(1_000)]
		pub fn increase_reputation(
			origin: OriginFor<T>,
			target: <T::Lookup as StaticLookup>::Source,
			reputation: u32,
		) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let sender = ensure_signed(origin)?;
			let target = T::Lookup::lookup(target)?;

			// Verify reputation
			ensure!(reputation <= T::MaxReputationChange::get(), Error::<T>::ReputationTooHigh);
			ensure!(reputation > 0, Error::<T>::ReputationTooLow);

			// Get current reputation for account
			let current_reputation = Reputations::<T>::get(&target).unwrap_or_else(Zero::zero);

			// Increase reputation
			let new_reputation = current_reputation + reputation;

			// Store reputation
			Reputations::<T>::insert(&target, new_reputation);

			// Emit an event that the claim was created.
			Self::deposit_event(Event::ReputationIncreased(sender, target, reputation));

			Ok(())
		}


		#[pallet::weight(1_000)]
		pub fn decrease_reputation(
			origin: OriginFor<T>,
			target: <T::Lookup as StaticLookup>::Source,
			reputation: u32,
		) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let sender = ensure_signed(origin)?;
			let target = T::Lookup::lookup(target)?;

			// Verify reputation
			ensure!(reputation <= T::MaxReputationChange::get(), Error::<T>::ReputationTooHigh);
			ensure!(reputation > 0, Error::<T>::ReputationTooLow);

			// Get current reputation for account
			let current_reputation = Reputations::<T>::get(&target).ok_or(Error::<T>::NoReputationToDecrease)?;

			// Decrease reputation
			let new_reputation = if current_reputation <= reputation {0} else {current_reputation - reputation};

			// Store reputation
			Reputations::<T>::insert(&target, new_reputation);

			// Emit an event that the claim was created.
			Self::deposit_event(Event::ReputationDecreased(sender, target, reputation));

			Ok(())
		}

	}
}

#[cfg(test)]
mod tests {
	// Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
	use crate as pallet_reputation;
	use frame_support::{
		assert_noop, assert_ok,
		traits::{ConstU32, ConstU64},
	};
	use sp_core::H256;
	use sp_runtime::{
		testing::Header,
		traits::{BlakeTwo256, IdentityLookup},
	};

	type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
	type Block = frame_system::mocking::MockBlock<Test>;

	frame_support::construct_runtime!(
		pub enum Test where
			Block = Block,
			NodeBlock = Block,
			UncheckedExtrinsic = UncheckedExtrinsic,
		{
			System: frame_system,
			Reputation: pallet_reputation,
		}
	);
	   
	impl frame_system::Config for Test {
		type BaseCallFilter = frame_support::traits::Everything;
		type BlockWeights = ();
		type BlockLength = ();
		type DbWeight = ();
		type Origin = Origin;
		type Index = u64;
		type BlockNumber = u64;
		type Hash = H256;
		type Call = Call;
		type Hashing = BlakeTwo256;
		type AccountId = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type Event = Event;
		type BlockHashCount = ConstU64<250>;
		type Version = ();
		type PalletInfo = PalletInfo;
		type AccountData = ();
		type OnNewAccount = ();
		type OnKilledAccount = ();
		type SystemWeightInfo = ();
		type SS58Prefix = ();
		type OnSetCode = ();
		type MaxConsumers = ConstU32<16>;
	}

	impl Config for Test {
		type MaxReputationChange = ConstU32<10>;
		type Event = Event;
	}

	#[derive(Default)]
	pub struct ExtBuilder;
	impl ExtBuilder {
		pub fn build(self) -> sp_io::TestExternalities {
			let t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
			let mut ext = sp_io::TestExternalities::new(t);
			ext.execute_with(|| System::set_block_number(1));
			ext
		}
	}
	   
	#[test]
	fn should_increase_reputation() {
		ExtBuilder::default().build().execute_with(|| {
			assert_eq!(Reputations::<Test>::get(2), None);
			assert_ok!(Reputation::increase_reputation(Origin::signed(1), 2, 5));
			assert_eq!(Reputations::<Test>::get(2).unwrap(), 5);
			assert_ok!(Reputation::increase_reputation(Origin::signed(1), 2, 7));
			assert_eq!(Reputations::<Test>::get(2).unwrap(), 12);
		});
	}

	#[test]
	fn should_decrease_reputation() {
		ExtBuilder::default().build().execute_with(|| {
			assert_eq!(Reputations::<Test>::get(2), None);
			assert_ok!(Reputation::increase_reputation(Origin::signed(1), 2, 8));
			assert_eq!(Reputations::<Test>::get(2).unwrap(), 8);
			assert_ok!(Reputation::decrease_reputation(Origin::signed(1), 2, 7));
			assert_eq!(Reputations::<Test>::get(2).unwrap(), 1);
			assert_ok!(Reputation::decrease_reputation(Origin::signed(1), 2, 7));
			assert_eq!(Reputations::<Test>::get(2).unwrap(), 0);
		});
	}

	#[test]
	fn should_catch_errors() {
		ExtBuilder::default().build().execute_with(|| {
			assert_noop!(Reputation::increase_reputation(Origin::signed(1), 2, 0), Error::<Test>::ReputationTooLow);
			assert_noop!(Reputation::increase_reputation(Origin::signed(1), 2, 20), Error::<Test>::ReputationTooHigh);
			assert_noop!(Reputation::decrease_reputation(Origin::signed(1), 2, 5), Error::<Test>::NoReputationToDecrease);
		});
	}
}