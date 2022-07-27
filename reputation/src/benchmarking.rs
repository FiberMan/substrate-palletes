//! Benchmarking setup for pallet-reputation

use crate::*;
use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

const SEED: u32 = 0;

benchmarks! {
    increase_reputation {
        let caller: T::AccountId = whitelisted_caller();
        let recipient: T::AccountId = account("recipient", 0, SEED);
		let recipient_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(recipient.clone());
        let s in 1 .. 10;
    }: increase_reputation(RawOrigin::Signed(caller), recipient_lookup, s.into())

    decrease_reputation {
        let caller: T::AccountId = whitelisted_caller();
        let recipient: T::AccountId = account("recipient", 0, SEED);
		let recipient_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(recipient.clone());
        let s in 1 .. 10;
    }: decrease_reputation(RawOrigin::Signed(caller), recipient_lookup, s.into())

	// impl_benchmark_test_suite!(Reputation, crate::mock::new_test_ext(), crate::mock::Test);
    impl_benchmark_test_suite!(Reputation, crate::mock::ExtBuilder::default().build(), crate::mock::Test);
}
