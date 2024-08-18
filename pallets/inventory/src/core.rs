use crate::{pallet::Pallet, types::Item, Config, Value};
use frame::prelude::BoundedVec;
use pallet_timestamp::{self as timestamp};
use sp_core::ConstU32;
use sp_runtime::DispatchResult; // Add this import

impl<T: Config> Pallet<T> {
    pub fn do_inventory_insertion(
        who: &T::AccountId,
        sku: &BoundedVec<u8, ConstU32<16>>,
        qty: u32,
        weight: u32,
        shelf_life: u32,
    ) -> DispatchResult {
        let item: Item<T> = Item {
            sku: sku.clone(),
            moved_by: who.clone(),
            qty,
            weight,
            shelf_life,
            created_at: <timestamp::Pallet<T>>::get(),
        };

        // Fetch the existing vector of items for the account
        let mut items = <Value<T>>::get(who).unwrap_or_default();

        // Add the new item to the vector
        items.push(item);

        // Store the updated vector back in storage
        <Value<T>>::insert(who, items);

        Ok(())
    }
}
