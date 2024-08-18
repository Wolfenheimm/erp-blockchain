use crate::Config;
use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_core::RuntimeDebug;

// ABC code selections
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, Default)]
pub enum AbcCode {
    #[default]
    A,
    B,
    C,
}

// Define the Item struct
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, Default, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct Item<T: Config> {
    pub moved_by: T::AccountId,
    pub lot_number: u32,
    pub abc_code: AbcCode,
    pub inventory_type: u32,
    pub product_type: u32,
    pub qty: u32,
    pub weight: u32,
    pub shelf_life: u32,
    pub cycle_count: u32,
    pub created_at: T::Moment,
}
