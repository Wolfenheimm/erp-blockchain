use codec::{Encode, MaxEncodedLen};
use crate::{pallet::Pallet, Item, Sku, InventoryType, ProductType, AbcCode, Config, Value, Error};
use sp_runtime::DispatchResult;
use pallet_timestamp::{self as timestamp};
use crate::types::MovedByAccount;

impl<T: Config> Pallet<T> {
    pub fn do_inventory_insertion(
        who: &T::AccountId,
        sku: &Sku,
        moved_by: &MovedByAccount,
        lot_number: u32,
        abc_code: &AbcCode,
        inventory_type: &InventoryType,
        product_type: &ProductType,
        qty: u32,
        weight: u32,
        cycle_count: u32,
        shelf_life: u32,
    ) -> DispatchResult {
        let item: Item<T> = Item {
            moved_by: moved_by.clone(),
            lot_number,
            abc_code: abc_code.clone(),
            inventory_type: inventory_type.clone(),
            product_type: product_type.clone(),
            qty,
            weight,
            cycle_count,
            shelf_life,
            created_at: <timestamp::Pallet<T>>::get(),
        };

        // Ensure SKU length does not exceed 16
        let sku_encoded_len = sku.encode().len();
        let max_encoded_len = Sku::max_encoded_len();

        if sku_encoded_len > max_encoded_len {
            return Err(Error::<T>::InvalidSkuLength.into());
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
