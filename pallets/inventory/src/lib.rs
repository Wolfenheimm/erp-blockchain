//! A shell pallet built with [`frame`].
//!
//! To get started with this pallet, try implementing the guide in
//! <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html>
//TODO
// Add storage for SKUs and associated metadata --
// Transaction call to insert or delete SKUs --
// Emit events for these actions
// Add tests

#![cfg_attr(not(feature = "std"), no_std)]

mod core;
mod types;

pub use pallet::*;

#[frame::pallet(dev_mode)]
pub mod pallet {
    pub use crate::types::{Item, AbcCode, InventoryType, ProductType, Sku, MovedByAccount};
    use frame::prelude::*;
    use pallet_timestamp::{self as timestamp};
    use sp_std::vec::Vec;

    /// The configuration trait of a pallet. Mandatory. Allows a pallet to receive types at a
    /// later point from the runtime that wishes to contain it. It allows the pallet to be
    /// parameterized over both types and values.
    #[pallet::config]
    pub trait Config: frame_system::Config + timestamp::Config {
        type RuntimeEvent: IsType<<Self as frame_system::Config>::RuntimeEvent> + From<Event<Self>>;
    }

    #[pallet::error]
    pub enum Error<T> {
        ConversionFailed,
        InvalidSkuLength,
    }

    /// A mandatory struct in each pallet. All functions callable by external users (aka.
    /// transactions) must be attached to this type (see [`frame::pallet_macros::call`]). For
    /// convenience, internal (private) functions can also be attached to this type.
    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// The events that this pallet can emit.
    #[pallet::event]
    #[pallet::generate_deposit(pub(crate) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event emitted when a new random seed is available from the relay chain
        AddNewItem {
            sender: T::AccountId,
            sku: Sku,
            moved_by: MovedByAccount,
            lot_number: u32,
            abc_code: AbcCode,
            inventory_type: InventoryType,
            product_type: ProductType,
            qty: u32,
            weight: u32,
            cycle_count: u32,
            shelf_life: u32,
        },
    }

    /// A storage item that this pallet contains. This will be part of the state root trie
    /// of the blockchain.
    #[pallet::storage]
    pub type Value<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Blake2_128Concat,
        Sku,
        Vec<Item<T>>,
        OptionQuery,
    >;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        pub fn inventory_insertion(
            origin: OriginFor<T>,
            sku: Sku,
            moved_by: MovedByAccount,
            lot_number: u32,
            abc_code: AbcCode,
            inventory_type: InventoryType,
            product_type: ProductType,
            qty: u32,
            weight: u32,
            cycle_count: u32,
            shelf_life: u32,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Request storage of the new item
            Self::do_inventory_insertion(&who, &sku, &moved_by, lot_number, &abc_code, &inventory_type, &product_type, qty, weight, cycle_count, shelf_life)?;

            // Emit an event detailing a new Item has been entered
            Self::deposit_event(Event::AddNewItem {
                sender: who,
                sku,
                moved_by,
                lot_number,
                abc_code,
                inventory_type,
                product_type,
                qty,
                weight,
                cycle_count,
                shelf_life,
            });

            Ok(())
        }
    }
}
