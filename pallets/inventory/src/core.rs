use crate::{pallet::Pallet, types::Item, Config, Error, Value};
use frame_support::dispatch::DispatchResult;
use frame_support::BoundedVec;
use sp_core::ConstU32;

impl<T: Config> Pallet<T> {
    pub fn do_inventory_insertion(
        who: &T::AccountId,
        sku: &BoundedVec<u8, ConstU32<16>>,
        qty: u128,
        weight: u128,
    ) -> DispatchResult {
        let item: Item<T> = Item {
            sku: sku.clone(),
            moved_by: who.clone(),
            qty,
            weight,
        };

        // Fetch the existing vector of items for the account
        let mut items = <Value<T>>::get(who).unwrap_or_else(|| BoundedVec::default());

        // Ensure we don't exceed the maximum capacity of the BoundedVec
        if items.len() >= items.capacity() {
            return Err(Error::<T>::ExceedsMaxItems.into());
        }

        // Add the new item to the BoundedVec
        items
            .try_push(item)
            .map_err(|_| Error::<T>::ExceedsMaxItems)?;

        // Store the updated vector back in storage
        <Value<T>>::insert(who, items);

        Ok(())
    }
}
