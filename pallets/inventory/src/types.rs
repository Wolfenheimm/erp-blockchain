use crate::Config;
use codec::{Decode, Encode};
use frame::prelude::{BoundedVec, ConstU32};
use scale_info::TypeInfo;
use sp_core::RuntimeDebug;

pub type Sku = BoundedVec<u8, ConstU32<16>>;
pub type LotNumber = BoundedVec<u8, ConstU32<16>>;
pub type SerialNumber = u32;
pub type Qty = u32;
pub type Weight = u32;
pub type ShelfLife = u32;
pub type CycleCount = u32;

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

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct Item<T: Config> {
    pub moved_by: MovedByAccount,
    pub sku: Sku,
    pub lot_number: LotNumber,
    pub serial_number: SerialNumber,
    pub abc_code: AbcCode,
    pub inventory_type: InventoryType,
    pub product_type: ProductType,
    pub qty: Qty,
    pub weight: Weight,
    pub shelf_life: ShelfLife,
    pub cycle_count: CycleCount,
    pub created_at: T::Moment,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct Lot {
    pub lot_number: LotNumber,
}
