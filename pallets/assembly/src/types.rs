use codec::{Decode, Encode};
use frame_support::pallet_prelude::ConstU32;
use frame_support::pallet_prelude::MaxEncodedLen;
use frame_support::BoundedVec;
use pallet_inventory::types::{Item, Recipe};
use scale_info::TypeInfo;

pub type WorkOrderNumber = u32;

#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Clone, PartialEq, Eq, Debug, Default)]
pub struct WorkOrder {
    pub work_order_number: WorkOrderNumber,
    pub recipe: Recipe,
}

#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Clone, PartialEq, Eq, Debug)]
pub struct Bom {
    pub materials: BoundedVec<Item, ConstU32<100>>,
}

impl Bom {
    /// Creates a new empty `Bom`
    pub fn new() -> Self {
        Bom {
            materials: BoundedVec::default(), // Initializes an empty BoundedVec
        }
    }
}
