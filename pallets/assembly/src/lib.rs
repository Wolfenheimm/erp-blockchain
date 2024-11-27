#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
mod blogic;
mod types;
pub mod weights;
pub use weights::*;

// Import the types and other modules from the inventory pallet
use crate::types::{Bom, WorkOrder, WorkOrderNumber};
use pallet_inventory::types::{Item, Location, SerialNumber, Sku};

// Define the pallet and its configuration
#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use pallet_inventory::pallet::Config as InventoryConfig;
    use pallet_inventory::types::{Equipment, Recipe};

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config + InventoryConfig {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type WeightInfo: WeightInfo;
    }

    #[pallet::storage]
    pub type AssembledProducts<T: Config> = StorageNMap<
        _,
        (
            NMapKey<Blake2_128Concat, T::AccountId>,
            NMapKey<Blake2_128Concat, Sku>,
            NMapKey<Blake2_128Concat, SerialNumber>,
        ),
        (Item, Bom),
        OptionQuery,
    >;

    #[pallet::storage]
    pub type StagingArea<T: Config> = StorageDoubleMap<
        _,
        Twox64Concat,
        WorkOrder, // First key: Work order
        Twox64Concat,
        Equipment,     // Second key: Equipment
        (Bom, Recipe), // Value: Bom
        OptionQuery,   // Query type
    >;

    #[pallet::storage]
    pub type WorkOrders<T: Config> =
        StorageMap<_, Twox64Concat, WorkOrderNumber, WorkOrder, OptionQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        PrepStaging {
            assembler: T::AccountId,
            work_order: WorkOrder,
        },
        /// An item has been assembled
        ProductAssembled {
            assembler: T::AccountId,
            work_order: WorkOrder,
        },
        AddWorkOrder {
            who: T::AccountId,
            work_order: WorkOrder,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Not enough components for assembly
        InsufficientComponents,
        /// Assembly process failed
        AssemblyFailed,
        ExceededMaxComponents,
        InsufficientInventory,
        DescriptionTooLong,
        WorkOrderNotFound,
        StagingAreaNotFound,
        BomConstructIssue,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Assemble a product from components in inventory
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        pub fn assemble_product(
            origin: OriginFor<T>,
            work_order: WorkOrder,
            serial_number: SerialNumber,
            staging_location: Location,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            Self::do_assemble_product(&who, work_order.clone(), serial_number, staging_location)?; // Recipe qty to be sent here instead
                                                                                                   // TODO: Think about what happens if the output quantity is less than the defined recipe quantity
                                                                                                   // -- It can be acceptable to have a worker manually adjust the item quantity after assembly, in case something went wrong.

            // Emit the assembled product
            Self::deposit_event(Event::ProductAssembled {
                assembler: who,
                work_order,
            });

            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        pub fn prepare_staging_area(origin: OriginFor<T>, work_order: WorkOrder) -> DispatchResult {
            let who = ensure_signed(origin)?;

            Self::do_prepare_staging_area(&who, work_order.clone())?;

            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        pub fn create_work_order(origin: OriginFor<T>, work_order: WorkOrder) -> DispatchResult {
            let who = ensure_signed(origin)?;

            Self::do_create_work_order(work_order.clone())?;

            // Emit staging has been prepped
            Self::deposit_event(Event::AddWorkOrder { who, work_order });

            Ok(())
        }
    }
}
