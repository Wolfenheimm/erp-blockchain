use crate::AccountId32;
use crate::{
    pallet::{self, Pallet},
    types::Item,
    Value,
};
use frame::prelude::{BoundedVec, DispatchError};
use sp_core::ConstU32; // Add this import

impl<T> Pallet<T>
where
    T: pallet::Config,
{
    pub(crate) fn do_request_storage(
        creator: sp_core::crypto::AccountId32,
        sku: BoundedVec<u8, ConstU32<16>>,
        qty: u32,
        weight: u32,
    ) -> Result<(), DispatchError> {
        let item = Item {
            sku,
            creator,
            qty,
            weight,
        };

        // Store the item in the blockchain storage
        //Value::<T>::put(item);
        Ok(())
    }
}
