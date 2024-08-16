use codec::{Decode, Encode, MaxEncodedLen};
use frame::prelude::BoundedVec;
use sp_core::ConstU32;
use crate::Config;


//pub type Sku = BoundedVec<u8, ConstU8<16>>;
// Ephemeral metadata of an inventory item.
#[derive(Encode, Decode, MaxEncodedLen, Debug, PartialEq, Eq, Clone)]
pub struct InventoryItemMetadata<T: Config> {
    // AccountId of the user who moved the inventory item to its destination.
    pub moved_by: T::AccountId,
    // The location of the item in inventory
    pub lot_number: T::LotNumber,
    // The item's SKU
    pub sku: BoundedVec<u8, ConstU32<16>>,
    // The item's quantity
    pub qty: T::Qty,
    // The item's weight
    pub weight: T::Weight,
    // The Item's date of purchase
    pub purchase_date: T::PurchaseDate,
    // The item's expiration date, if any
    pub expiration_date: T::ExpirationDate,
}