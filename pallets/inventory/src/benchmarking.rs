//! Benchmarking setup for pallet-inventory
#![cfg(feature = "runtime-benchmarks")]
use super::*;

use crate::types::{
    AbcCode, CycleCount, InventoryType, LotNumber, MovedByAccount, ProductType, Qty, SerialNumber,
    ShelfLife, Sku, WeightLbs,
};

#[allow(unused)]
use crate::Pallet as Template;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn inventory_insertion() {
        let caller: T::AccountId = whitelisted_caller();

        let sku = Sku::default();

        #[extrinsic_call]
        _(
            RawOrigin::Signed(caller.clone()),
            sku.clone(),
            MovedByAccount::default(),
            LotNumber::default(),
            SerialNumber::default(),
            AbcCode::default(),
            InventoryType::default(),
            ProductType::default(),
            Qty::default(),
            WeightLbs::default(),
            CycleCount::default(),
            ShelfLife::default(),
        );

        let key = (caller.clone(), sku);

        assert!(Inventory::<T>::contains_key(key));
    }

    impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
