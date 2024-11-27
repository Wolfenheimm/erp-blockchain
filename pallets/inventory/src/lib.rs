//! # Template Pallet
//!
//! A pallet with minimal functionality to help developers understand the essential components of
//! writing a FRAME pallet. It is typically used in beginner tutorials or in Substrate template
//! nodes as a starting point for creating a new pallet and **not meant to be used in production**.
//!
//! ## Overview
//!
//! This template pallet contains basic examples of:
//! - declaring a storage item that stores a single `u32` value
//! - declaring and using events
//! - declaring and using errors
//! - a dispatchable function that allows a user to set a new value to storage and emits an event
//!   upon success
//! - another dispatchable function that causes a custom error to be thrown
//!
//! Each pallet section is annotated with an attribute using the `#[pallet::...]` procedural macro.
//! This macro generates the necessary code for a pallet to be aggregated into a FRAME runtime.
//!
//! Learn more about FRAME macros [here](https://docs.substrate.io/reference/frame-macros/).
//!
//! ### Pallet Sections
//!
//! The pallet sections in this template are:
//!
//! - A **configuration trait** that defines the types and parameters which the pallet depends on
//!   (denoted by the `#[pallet::config]` attribute). See: [`Config`].
//! - A **means to store pallet-specific data** (denoted by the `#[pallet::storage]` attribute).
//!   See: [`storage_types`].
//! - A **declaration of the events** this pallet emits (denoted by the `#[pallet::event]`
//!   attribute). See: [`Event`].
//! - A **declaration of the errors** that this pallet can throw (denoted by the `#[pallet::error]`
//!   attribute). See: [`Error`].
//! - A **set of dispatchable functions** that define the pallet's functionality (denoted by the
//!   `#[pallet::call]` attribute). See: [`dispatchables`].
//!
//! Run `cargo doc --package pallet-template --open` to view this pallet's documentation.

// We make sure this pallet uses `no_std` for compiling to Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

// FRAME pallets require their own "mock runtimes" to be able to run unit tests. This module
// contains a mock runtime specific for testing this pallet's functionality.
#[cfg(test)]
mod mock;

// This module contains the unit tests for this pallet.
// Learn about pallet unit testing here: https://docs.substrate.io/test/unit-testing/
#[cfg(test)]
mod tests;

mod blogic;
pub mod types;

// Every callable function or "dispatchable" a pallet exposes must have weight values that correctly
// estimate a dispatchable's execution time. The benchmarking module is used to calculate weights
// for each dispatchable and generates this pallet's weight.rs file. Learn more about benchmarking here: https://docs.substrate.io/test/benchmark/
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

// All pallet logic is defined in its own module and must be annotated by the `pallet` attribute.
#[frame_support::pallet]
pub mod pallet {

    // Import various useful types required by all FRAME pallets.
    use super::*;
    use crate::WeightInfo;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use types::{SerialNumber, *};

    // The `Pallet` struct serves as a placeholder to implement traits, methods and dispatchables
    // (`Call`s) in this pallet.
    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// The pallet's configuration trait.
    ///
    /// All our types and constants a pallet depends on must be declared here.
    /// These types are defined generically and made concrete when the pallet is declared in the
    /// `runtime/src/lib.rs` file of your chain.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching runtime event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// A type representing the weights required by the dispatchables of this pallet.
        type WeightInfo: WeightInfo;
    }

    /// Events that functions in this pallet can emit.
    ///
    /// Events are a simple means of indicating to the outside world (such as dApps, chain explorers
    /// or other users) that some notable update in the runtime has occurred. In a FRAME pallet, the
    /// documentation for each event field and its parameters is added to a node's metadata so it
    /// can be used by external interfaces or tools.
    ///
    ///	The `generate_deposit` macro generates a function on `Pallet` called `deposit_event` which
    /// will convert the event type of your pallet into `RuntimeEvent` (declared in the pallet's
    /// [`Config`] trait) and deposit it using [`frame_system::Pallet::deposit_event`].
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        AddNewItem {
            sender: T::AccountId,
            item: Item,
        },
        ItemScrapped {
            sender: T::AccountId,
            sku: Sku,
            serial_number: SerialNumber,
            scrap_details: ScrapDetails,
        },
        MoveItem {
            sender: T::AccountId,
            item: Item,
            moved_by: Employee,
            adjust_details: AdjustDetails,
        },
        AdjustItem {
            sender: T::AccountId,
            item: Item,
            issuer: Employee,
            adjust_details: AdjustDetails,
        },
        AddRecipe {
            sender: T::AccountId,
            recipe: Recipe,
        },
        AddMaterial {
            sender: T::AccountId,
            material: Material,
        },
        DeleteMaterial {
            sender: T::AccountId,
            sku: Sku,
        },
        UpdateMaterial {
            sender: T::AccountId,
            original_material: Material,
            new_material: Material,
        },
    }

    /// Global Inventory Storage
    /// This storage is used to store the total quantity of each SKU in the inventory.
    #[pallet::storage]
    pub type GlobalInventory<T: Config> = StorageMap<_, Twox64Concat, Sku, Qty>;

    /// Inventory Storage
    /// This storage is used to store items in the inventory.
    #[pallet::storage]
    pub type Inventory<T: Config> = StorageNMap<
        _,
        (
            NMapKey<Blake2_128Concat, T::AccountId>,
            NMapKey<Blake2_128Concat, Sku>,
            NMapKey<Blake2_128Concat, SerialNumber>,
        ),
        Item,
        OptionQuery,
    >;

    /// Scrap Inventory Storage
    /// This storage is used to store items that are defective, damaged, or otherwise unusable.
    /// The items in this storage are not part of the main inventory and are not counted as part of the stock.
    #[pallet::storage]
    pub type ScrapInventory<T: Config> = StorageNMap<
        _,
        (
            NMapKey<Blake2_128Concat, T::AccountId>,
            NMapKey<Blake2_128Concat, Sku>,
            NMapKey<Blake2_128Concat, SerialNumber>,
        ),
        ScrapItem,
        OptionQuery,
    >;

    /// Adjust Inventory Storage
    /// This storage is used to store qty adjustments made to items in the inventory.
    #[pallet::storage]
    pub type AdjustInventory<T: Config> = StorageNMap<
        _,
        (
            NMapKey<Blake2_128Concat, T::AccountId>,
            NMapKey<Blake2_128Concat, Sku>,
            NMapKey<Blake2_128Concat, SerialNumber>,
        ),
        AdjustItem,
        OptionQuery,
    >;

    /// Recipes Storage
    /// This storage is used to store the recipes for assembling products.
    #[pallet::storage]
    pub type Recipes<T: Config> = StorageMap<_, Twox64Concat, Sku, Recipe>;

    #[pallet::storage]
    pub type Materials<T: Config> = StorageMap<_, Twox64Concat, Sku, Material>;

    #[pallet::storage]
    pub type InventoryLocale<T: Config> =
        StorageMap<_, Twox64Concat, Location, BoundedBTreeMap<SerialNumber, Item, ConstU32<1000>>>;

    /// Errors that can be returned by this pallet.
    ///
    /// Errors tell users that something went wrong so it's important that their naming is
    /// informative. Similar to events, error documentation is added to a node's metadata so it's
    /// equally important that they have helpful documentation associated with them.
    ///
    /// This type of runtime error can be up to 4 bytes in size should you want to return additional
    /// information.
    #[pallet::error]
    pub enum Error<T> {
        /// The value retrieved was `None` as no value was previously set.
        NoneValue,
        /// There was an attempt to increment the value in storage over `u32::MAX`.
        StorageOverflow,
        /// The SKU length is invalid
        InvalidSkuLength,
        /// The SKU is not in inventory
        InventoryNotFound,
        /// The inventory is insufficient
        InsufficientInventory,
        /// The incorrect Adjust Details were provided
        InvalidAdjustDetails,
        /// The inventory is full
        InventoryFull,
        /// The location was not found
        LocationNotFound,
        /// The Material already exists
        MaterialAlreadyExists,
        /// The material was not found
        MaterialNotFound,
    }

    /// The pallet's dispatchable functions ([`Call`]s).
    ///
    /// Dispatchable functions allows users to interact with the pallet and invoke state changes.
    /// These functions materialize as "extrinsics", which are often compared to transactions.
    /// They must always return a `DispatchResult` and be annotated with a weight and call index.
    ///
    /// The [`call_index`] macro is used to explicitly
    /// define an index for calls in the [`Call`] enum. This is useful for pallets that may
    /// introduce new dispatchables over time. If the order of a dispatchable changes, its index
    /// will also change which will break backwards compatibility.
    ///
    /// The [`weight`] macro is used to assign a weight to each call.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Insert inventory item into storage
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::inventory_insertion())]
        pub fn inventory_insertion(origin: OriginFor<T>, item: Item) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Insert the item into storage
            Self::do_inventory_insertion(&who, item.clone())?;

            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        pub fn inventory_scrap(
            origin: OriginFor<T>,
            sku: Sku,
            serial_number: SerialNumber,
            scrap_details: ScrapDetails,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Scrap the item, if it exists
            Self::do_inventory_scrap(&who, sku.clone(), serial_number, scrap_details.clone())?;

            // Emit the scrap
            Self::deposit_event(Event::ItemScrapped {
                sender: who,
                sku,
                serial_number,
                scrap_details,
            });

            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        pub fn inventory_move(
            origin: OriginFor<T>,
            sku: Sku,
            serial_number: SerialNumber,
            moved_by: Employee,
            adjust_details: AdjustDetails,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Check if the item exists in inventory
            let item = Inventory::<T>::get((&who, sku.clone(), serial_number))
                .ok_or(Error::<T>::InventoryNotFound)?;

            // Move the item to the new location
            Self::do_inventory_move(&who, item.clone(), moved_by.clone(), adjust_details.clone())?;

            // Emit the move
            Self::deposit_event(Event::MoveItem {
                sender: who,
                item: item.clone(),
                moved_by,
                adjust_details,
            });

            Ok(())
        }

        #[pallet::call_index(3)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        pub fn inventory_adjust(
            origin: OriginFor<T>,
            issuer: Employee,
            sku: Sku,
            serial_number: SerialNumber,
            adjust_details: AdjustDetails,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Check if the item exists in inventory
            let item = Inventory::<T>::get((&who, sku.clone(), serial_number))
                .ok_or(Error::<T>::InventoryNotFound)?;

            // Adjust the item's quantity
            Self::do_inventory_adjust(&who, issuer.clone(), item.clone(), adjust_details.clone())?;

            Ok(())
        }

        #[pallet::call_index(4)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        pub fn insert_recipe(origin: OriginFor<T>, recipe: Recipe) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Adjust the item's quantity
            Self::do_insert_recipe(recipe.clone())?;

            // Emit the adjustment
            Self::deposit_event(Event::AddRecipe {
                sender: who,
                recipe,
            });

            Ok(())
        }

        #[pallet::call_index(5)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        pub fn insert_material(origin: OriginFor<T>, material: Material) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Adjust the item's quantity
            Self::do_insert_material(material.clone())?;

            Self::deposit_event(Event::AddMaterial {
                sender: who,
                material,
            });

            Ok(())
        }

        #[pallet::call_index(6)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        pub fn delete_material(origin: OriginFor<T>, sku: Sku) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Adjust the item's quantity
            Self::do_delete_material(sku.clone())?;

            Self::deposit_event(Event::DeleteMaterial { sender: who, sku });

            Ok(())
        }

        #[pallet::call_index(7)]
        #[pallet::weight(T::WeightInfo::inventory_insertion())]
        pub fn update_material(origin: OriginFor<T>, material: Material) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Fetch the original material
            let original_material =
                <Materials<T>>::get(&material.sku).ok_or(Error::<T>::MaterialNotFound)?;

            // Adjust the item's quantity
            Self::do_update_material(material.clone())?;

            Self::deposit_event(Event::UpdateMaterial {
                sender: who,
                original_material,
                new_material: material,
            });

            Ok(())
        }
    }
}
