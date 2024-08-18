use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::BoundedVec;
use frame_system::Config;
use scale_info::TypeInfo;
use sp_core::{ConstU32, RuntimeDebug};

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, Default, MaxEncodedLen, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct Item<T: Config> {
    pub sku: BoundedVec<u8, ConstU32<16>>,
    pub moved_by: T::AccountId,
    pub qty: u128,
    pub weight: u128,
}
