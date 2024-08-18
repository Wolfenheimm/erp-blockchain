use frame::prelude::{BoundedVec, ConstU32};
use crate::{pallet::Pallet, types::Item, Config, Value, Error};
use sp_runtime::DispatchResult;
use pallet_timestamp::{self as timestamp};

impl<T: Config> Pallet<T> {
    pub fn do_inventory_insertion(
        who: &T::AccountId,
        sku: &T::Sku,
        lot_number: u32,
        abc_code: &BoundedVec<u8, ConstU32<1>>,
        inventory_type: u32,
        product_type: u32,
        qty: u32,
        weight: u32,
        cycle_count: u32,
        shelf_life: u32,
    ) -> DispatchResult {
        let item: Item<T> = Item {
            moved_by: who.clone(),
            lot_number,
            abc_code: abc_code.clone(),
            inventory_type,
            product_type,
            qty,
            weight,
            cycle_count,
            shelf_life,
            created_at: <timestamp::Pallet<T>>::get(),
        };

        if !is_valid_abc_code(abc_code) {
            return Err(Error::<T>::InvalidAbcCode.into());
        }

        // Fetch the existing vector of items for the account
        let mut items = <Value<T>>::get(who, sku).unwrap_or_default();

        // Add the new item to the vector
        items.push(item);

        // Store the updated vector back in storage
        <Value<T>>::insert(who, sku, items);

        Ok(())
    }
}

fn is_valid_abc_code(abc_code: &BoundedVec<u8, ConstU32<1>>) -> bool {
    match abc_code.as_slice() {
        [b'A'] | [b'B'] | [b'C'] => true,
        _ => false,
    }
}
