//! Reputation Pallet
#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;
pub use weights::WeightInfo;

use sp_runtime::traits::{StaticLookup, Zero};

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
		/// Information on runtime weights.
		type WeightInfo: WeightInfo;
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
		#[pallet::weight(T::WeightInfo::increase_reputation())]
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

		#[pallet::weight(T::WeightInfo::decrease_reputation())]
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
			let current_reputation = Reputations::<T>::get(&target).unwrap_or_else(Zero::zero);

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