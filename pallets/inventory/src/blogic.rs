use crate::Config;
use crate::Error;
use crate::Inventory;
use crate::{pallet::Pallet, types::*};
use codec::{Encode, MaxEncodedLen};
use frame_support::sp_runtime::DispatchResult;
use frame_support::BoundedVec;

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
        weight_lbs: &WeightLbs,
        cycle_count: &CycleCount,
        shelf_life: &ShelfLife,
    ) -> DispatchResult {
        let item: Item = Item {
            moved_by: moved_by.clone(),
            sku: sku.clone(),
            lot_number: lot_number.clone(),
            serial_number: serial_number.clone(),
            abc_code: abc_code.clone(),
            inventory_type: inventory_type.clone(),
            product_type: product_type.clone(),
            qty: qty.clone(),
            weight: weight_lbs.clone(),
            cycle_count: cycle_count.clone(),
            shelf_life: shelf_life.clone(),
            created_at: 1,
        };

        // Ensure SKU length does not exceed 16
        let sku_encoded_len = sku.encode().len();
        let max_encoded_len = Sku::max_encoded_len();

        if sku_encoded_len > max_encoded_len {
            return Err(Error::<T>::InvalidSkuLength.into());
        }

        // Fetch the existing inventory for the (who, sku) combination
        let mut items = <Inventory<T>>::get((who, sku)).unwrap_or_else(|| BoundedVec::new());

        // Try to push the new item to the vector
        items
            .try_push(item)
            .map_err(|_| Error::<T>::StorageOverflow)?;

        // Insert the updated vector back into storage
        <Inventory<T>>::insert((who, sku), items);

        Ok(())
    }
}
