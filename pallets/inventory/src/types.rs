use crate::Config;
use codec::{Decode, Encode};
use frame::prelude::{BoundedVec, ConstU32};
use scale_info::TypeInfo;
use sp_core::RuntimeDebug;

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, Default)]
pub enum AbcCode {
    #[default]
    A,
    B,
    C,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, Default)]
pub enum InventoryType {
    #[default]
    RawMaterial,
    Component,
    WIP,
    FinishedGood,
    MRO,
    PackagingMaterials,
    SafetyAnticipationStock,
    Decoupling,
    Cycle,
    Service,
    Transit,
    Theoretical,
    Excess,
}

/// Define an enum for selectable accounts
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, Default)]
pub enum MovedByAccount {
    #[default]
    Bob,
    Charlie,
    Dave,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, Default)]
pub enum ProductType {
    #[default]
    CapitalGoods,
    RawMaterials,
    ComponentParts,
    MajorEquipment,
    AccessoryEquipment,
    OperatingSupplies,
}

pub type Sku = BoundedVec<u8, ConstU32<16>>;

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct Item<T: Config> {
    pub moved_by: MovedByAccount,
    pub lot_number: u32,
    pub abc_code: AbcCode,
    pub inventory_type: InventoryType,
    pub product_type: ProductType,
    pub qty: u32,
    pub weight: u32,
    pub shelf_life: u32,
    pub cycle_count: u32,
    pub created_at: T::Moment,
}
