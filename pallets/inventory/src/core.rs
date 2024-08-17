use crate::{pallet::Pallet, types::Item, Config, Value};
use frame::prelude::BoundedVec;
use sp_core::ConstU32;
use sp_runtime::DispatchResult; // Add this import

impl<T: Config> Pallet<T> {
    pub fn do_inventory_insertion(
        who: &T::AccountId,
        sku: &BoundedVec<u8, ConstU32<16>>,
        qty: u128,
        weight: u128,
    ) -> DispatchResult {
        // Put the all the data into storage
        // The `Value` storage item is used to store an Item

        let item: Item<T> = Item {
            sku: sku.clone(),
            moved_by: who.clone(),
            qty,
            weight,
        };

        <Value<T>>::put(item);
        Ok(())
    }
}
