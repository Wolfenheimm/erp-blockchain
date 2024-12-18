//! # Assembly Pallet
//!
//! - [`Config`]
//! - [`Call`]
//!
//! ## Overview
//!
//! The Assembly pallet facilitates product assembly and work order management within the runtime.
//! It interacts with inventory management to stage, track, and assemble items.
//! This pallet provides functionalities to:
//!
//! - Prepare staging areas for product assembly.
//! - Create and manage work orders.
//! - Assemble products using predefined recipes and components.
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! - `assemble_product`: Assemble a product using inventory components and a work order.
//! - `prepare_staging_area`: Prepare a staging area for a specific work order.
//! - `create_work_order`: Create a new work order for assembly.
//!
//! ## Storage
//!
//! - `AssembledProducts`: Tracks completed products, including their Bill of Materials (BOM).
//! - `StagingArea`: Stages components and equipment for specific work orders.
//! - `WorkOrders`: Stores work orders for tracking and reference.
//!
//! ## Events
//!
//! - `ProductAssembled`: Emitted when a product is successfully assembled.
//! - `PrepStaging`: Emitted when a staging area is prepared for assembly.
//! - `AddWorkOrder`: Emitted when a new work order is added to the system.
//!
//! ## Errors
//!
//! - `InsufficientComponents`: Not enough components for assembly.
//! - `AssemblyFailed`: General failure during the assembly process.
//! - `WorkOrderNotFound`: The specified work order could not be located.
//! - `StagingAreaNotFound`: Staging area for the work order is missing.
//! - `ExceededMaxComponents`: The maximum number of components has been exceeded.
//! - `InsufficientInventory`: Not enough inventory to complete the assembly.
//! - `DescriptionTooLong`: The description is too long.
//! - `WorkOrderNotFound`: The specified work order could not be located.
//! - `BomConstructIssue`: Issue with the construction of the Bill of Materials.
//!
//! This pallet uses `no_std` for compatibility with Wasm environments, a polkadot standard.
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

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// The pallet's configuration trait.
    ///
    /// This trait is tightly coupled with the Inventory pallet -> [InventoryConfig]
    #[pallet::config]
    pub trait Config: frame_system::Config + InventoryConfig {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type WeightInfo: WeightInfo;
    }

    /// Assembled Products Storage
    ///
    /// All Items that have been assembled
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

    /// Work Order Storage
    ///
    /// Work orders that are in progress or have not yet been completed
    #[pallet::storage]
    pub type WorkOrders<T: Config> =
        StorageMap<_, Twox64Concat, WorkOrderNumber, WorkOrder, OptionQuery>;

    /// Events that functions in this pallet can emit.
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

    /// Errors that can be returned by this pallet.
    ///
    /// This type of runtime error can be up to 4 bytes in size should you want to return additional
    /// information.
    #[pallet::error]
    pub enum Error<T> {
        /// Not enough components for assembly
        InsufficientComponents,
        /// Assembly process failed
        AssemblyFailed,
        /// Not enough inventory to set up staging or assemble the product
        InsufficientInventory,
        /// The description length exceeds the maximum imposed limit
        DescriptionTooLong,
        /// Unable to find the defined work order
        WorkOrderNotFound,
        /// Unable to find the defined staging area
        StagingAreaNotFound,
        /// The BOM was improperly built
        BomConstructIssue,
        /// The work order already exists
        WorkOrderAlreadyExists,
    }

    /// The pallet's dispatchable functions
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Assemble a product from components in staging
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        pub fn assemble_product(
            origin: OriginFor<T>,
            work_order: WorkOrder,
            serial_number: SerialNumber,
            staging_location: Location,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            Self::do_assemble_product(&who, work_order.clone(), serial_number, staging_location)?;

            // Emit the assembled product
            Self::deposit_event(Event::ProductAssembled {
                assembler: who,
                work_order,
            });

            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        /// Prepare the staging area for assembly
        /// TODO: Later, add location to define different staging areas
        pub fn prepare_staging_area(origin: OriginFor<T>, work_order: WorkOrder) -> DispatchResult {
            let who = ensure_signed(origin)?;

            Self::do_prepare_staging_area(&who, work_order.clone())?;

            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        /// Create a new work order
        pub fn create_work_order(origin: OriginFor<T>, work_order: WorkOrder) -> DispatchResult {
            let who = ensure_signed(origin)?;

            Self::do_create_work_order(work_order.clone())?;

            // Emit staging has been prepped
            Self::deposit_event(Event::AddWorkOrder { who, work_order });

            Ok(())
        }
    }
}
