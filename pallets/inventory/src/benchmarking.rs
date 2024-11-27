//! Benchmarking setup for pallet-inventory
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as Template;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

#[benchmarks]
mod benchmarks {
    use types::Item;

    use super::*;

    #[benchmark]
    fn inventory_insertion() {
        let caller: T::AccountId = whitelisted_caller();

        #[extrinsic_call]
        _(RawOrigin::Signed(caller.clone()), Item::default());

        let key = (
            caller.clone(),
            Item::default().sku,
            Item::default().serial_number,
        );

        assert!(Inventory::<T>::contains_key(key));
    }

    impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
