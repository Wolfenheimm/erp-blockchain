use codec::{Decode, Encode};
use frame_support::pallet_prelude::MaxEncodedLen;
use frame_support::sp_runtime::RuntimeDebug;
use frame_support::traits::ConstU32;
use frame_support::BoundedVec;
use scale_info::TypeInfo;

pub type CycleCount = u32;
pub type ShelfLife = u32;
pub type LotNumber = u32;
pub type SerialNumber = u32;
pub type ExpirationDate = u32;
pub type ProductionDate = u32;
pub type WeightLbs = u32;
pub type PurchaseDate = u32;
pub type Qty = u32;
pub type Sku = BoundedVec<u8, ConstU32<16>>;

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, Default, MaxEncodedLen)]
pub enum AbcCode {
    #[default]
    A,
    B,
    C,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, Default, MaxEncodedLen)]
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

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, Default, MaxEncodedLen)]
pub enum MovedByAccount {
    #[default]
    Bob,
    Charlie,
    Dave,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, Default, MaxEncodedLen)]
pub enum ProductType {
    #[default]
    CapitalGoods,
    RawMaterials,
    ComponentParts,
    MajorEquipment,
    AccessoryEquipment,
    OperatingSupplies,
}

#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Clone, PartialEq, Eq, Debug)]
pub struct Item {
    pub moved_by: MovedByAccount,
    pub sku: Sku,
    pub lot_number: LotNumber,
    pub serial_number: SerialNumber,
    pub abc_code: AbcCode,
    pub inventory_type: InventoryType,
    pub product_type: ProductType,
    pub qty: Qty,
    pub weight: WeightLbs,
    pub shelf_life: ShelfLife,
    pub cycle_count: CycleCount,
    pub created_at: u32,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct Lot {
    pub lot_number: LotNumber,
}
