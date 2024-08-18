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

pub use pallet::*;

mod core;
mod types;

#[frame_support::pallet]
pub mod pallet {
    pub use crate::types::Item;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    /// The configuration trait of a pallet. Mandatory. Allows a pallet to receive types at a
    /// later point from the runtime that wishes to contain it. It allows the pallet to be
    /// parameterized over both types and values.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: IsType<<Self as frame_system::Config>::RuntimeEvent> + From<Event<Self>>;
        type LotNumber: Get<u32>;
        type Qty: Get<u32>;
        type Weight: Get<u32>;
        type PurchaseDate: Get<u32>;
        type ExpirationDate: Get<u32>;

        /// Maximum number of items in the vector
        #[pallet::constant]
        type MaxItems: Get<u32>;
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Error indicating that the conversion failed.
        ConversionFailed,
        ExceedsMaxItems, // Other error variants...
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    pub type Value<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, BoundedVec<Item<T>, ConstU32<100>>>;

    /// All *dispatchable* call functions (aka. transactions) are attached to `Pallet` in a
    /// `impl` block.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        pub fn inventory_insertion(
            origin: OriginFor<T>,
            sku: BoundedVec<u8, ConstU32<16>>,
            qty: u128,
            weight: u128,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Call the core logic from core.rs
            Pallet::<T>::do_inventory_insertion(&who, &sku, qty, weight)
        }
    }

    /// The events that this pallet can emit.
    #[pallet::event]
    #[pallet::generate_deposit(pub(crate) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event emitted when a new random seed is available from the relay chain
        AddNewItem {
            //random_hash: BoundedVec<u8, ConstU8>,
            sender: T::AccountId,
            sku: BoundedVec<u8, ConstU32<16>>,
            qty: u128,
            weight: u128,
        },
    }
}
