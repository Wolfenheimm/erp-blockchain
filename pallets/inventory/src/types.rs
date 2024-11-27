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
pub type Reason = BoundedVec<u8, ConstU32<128>>;

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, Default, MaxEncodedLen)]
pub enum AbcCode {
    #[default]
    A,
    B,
    C,
}

#[derive(Default, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum Equipment {
    #[default]
    Forklift,
    Crane,
    Conveyor,
    Truck,
    PalletJack,
    HandTruck,
    Cart,
    Crimper,
    Cutter,
    Palletizer,
    Mixer,
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
pub enum Employee {
    #[default]
    Bob,
    Charlie,
    Dave,
    Auto,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, Default, MaxEncodedLen)]
pub enum Location {
    #[default]
    Warehouse,
    Production,
    Shipping,
    Receiving,
    Scrap,
    Staging,
    Packaging,
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

#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Clone, PartialEq, Eq, Debug, Default)]
pub struct Item {
    pub moved_by: Employee,
    pub sku: Sku,
    pub lot_number: LotNumber,
    pub serial_number: SerialNumber,
    pub material: Material,
    pub abc_code: AbcCode,
    pub inventory_type: InventoryType,
    pub product_type: ProductType,
    pub qty: Qty,
    pub weight: WeightLbs,
    pub shelf_life: ShelfLife,
    pub cycle_count: CycleCount,
    pub created_at: u32,
    pub location: Location,
}

#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Clone, PartialEq, Eq, Debug, Default)]
pub struct Material {
    pub sku: Sku,
}

#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Clone, PartialEq, Eq, Debug)]
pub struct ScrapItem {
    pub item: Item,
    pub details: ScrapDetails,
}

#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Clone, PartialEq, Eq, Debug)]
pub struct ScrapDetails {
    pub issuer: Employee,
    pub reason: Reason,
    pub equipment: Equipment,
}

#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Clone, PartialEq, Eq, Debug)]
pub struct AdjustItem {
    pub issuer: Employee,
    pub item: Item,
    pub adjust_details: AdjustDetails,
}

#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Clone, PartialEq, Eq, Debug)]
pub enum AdjustDetails {
    Quantity {
        original_qty: Qty,
        new_qty: Qty,
        reason: Reason,
    },
    Location {
        original_location: Location,
        new_location: Location,
        reason: Reason,
    },
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct Lot {
    pub lot_number: LotNumber,
}

// make an AssemblyDetails struct that defines the details to assemble a product
#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Clone, PartialEq, Eq, Debug, Default)]
pub struct Recipe {
    pub inserted_by: Employee,
    pub sku: Sku,
    pub recipe_id: u32,
    pub required_components: BoundedVec<RecipeComponent, ConstU32<100>>,
    pub required_equipment: Equipment,
    pub output_quantity: u32,
}

#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Clone, PartialEq, Eq, Debug)]
pub struct RecipeComponent {
    pub sku: Sku,
    pub qty: u32,
}
