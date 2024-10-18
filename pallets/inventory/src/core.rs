use codec::{Encode, MaxEncodedLen};
use crate::{pallet::Pallet, Item, Sku, InventoryType, ProductType, AbcCode, Config, Value, Error};
use sp_runtime::DispatchResult;
use pallet_timestamp::{self as timestamp};
use crate::types::{Lot, CycleCount, LotNumber, MovedByAccount, Qty, SerialNumber, ShelfLife, Weight};
use scale_info::prelude::vec;


impl<T: Config> Pallet<T> {
    pub fn do_inventory_insertion(
        who: &T::AccountId,
        sku: &Sku,
        moved_by: &MovedByAccount,
        lot_number: &LotNumber,
        serial_number: &SerialNumber,
        abc_code: &AbcCode,
        inventory_type: &InventoryType,
        product_type: &ProductType,
        qty: &Qty,
        weight: &Weight,
        cycle_count: &CycleCount,
        shelf_life: &ShelfLife,
    ) -> DispatchResult {
        let item: Item<T> = Item {
            moved_by: moved_by.clone(),
            sku: sku.clone(),
            lot_number: lot_number.clone(),
            serial_number: serial_number.clone(),
            abc_code: abc_code.clone(),
            inventory_type: inventory_type.clone(),
            product_type: product_type.clone(),
            qty: qty.clone(),
            weight: weight.clone(),
            cycle_count: cycle_count.clone(),
            shelf_life: shelf_life.clone(),
            created_at: <timestamp::Pallet<T>>::get(),
        };

        let lot = Lot {
            lot_number: lot_number.clone(),
        };

        // Ensure SKU length does not exceed 16
        let sku_encoded_len = sku.encode().len();
        let max_encoded_len = Sku::max_encoded_len();

        if sku_encoded_len > max_encoded_len {
            return Err(Error::<T>::InvalidSkuLength.into());
        }

        let mut items = <Value<T>>::get((who, sku)).unwrap_or_default();

        // Check if there's an existing lot with the given lot_number
        let mut found_lot = false;

        // Iterate through items to find the lot and update it
        for (existing_lot, ref mut existing_items) in items.iter_mut() {
            if &existing_lot.lot_number == lot_number {
                existing_items.push(item.clone());
                found_lot = true;
                break;
            }
        }

        if !found_lot {
            // If no existing lot found, create a new lot and add the item
            items.push((Lot { lot_number: lot_number.clone() }, vec![item]));
        }

        <Value<T>>::insert((who, sku), items);

        Ok(())
    }
}
