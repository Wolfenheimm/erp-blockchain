use crate::Config;
use codec::{Decode, Encode, MaxEncodedLen};
use frame::prelude::BoundedVec;
use scale_info::TypeInfo;
use sp_core::{ConstU32, RuntimeDebug};

// Define the Item struct
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, Default, MaxEncodedLen, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct Item<T: Config> {
    pub sku: BoundedVec<u8, ConstU32<16>>,
    pub moved_by: T::AccountId,
    pub qty: u32,
    pub weight: u32,
    pub shelf_life: u32,
    pub created_at: T::Moment,
}
