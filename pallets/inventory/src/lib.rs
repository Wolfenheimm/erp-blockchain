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

// Re-export all pallet parts, this is needed to properly import the pallet into the runtime.

pub use pallet::*;
pub use sp_core::crypto::AccountId32;

#[frame::pallet(dev_mode)]
pub mod pallet {
    use crate::AccountId32;
    use frame::prelude::*;

    /// The configuration trait of a pallet. Mandatory. Allows a pallet to receive types at a
    /// later point from the runtime that wishes to contain it. It allows the pallet to be
    /// parameterized over both types and values.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// A type that is not known now, but the runtime that will contain this pallet will
        /// know it later, therefore we define it here as an associated type.
        type RuntimeEvent: IsType<<Self as frame_system::Config>::RuntimeEvent> + From<Event<Self>>;

        /// A parameterize-able value that we receive later via the `Get<_>` trait.
        type LotNumber: Get<u32>;

        type Qty: Get<u32>;

        type Weight: Get<u32>;

        type PurchaseDate: Get<u32>;

        type ExpirationDate: Get<u32>;

        type AccountId: T::AccountId;
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
            //random_hash: BoundedVec<u8, ConstU8>,
            sender: AccountId32,
            sku: BoundedVec<u8, ConstU32<16>>,
            qty: u128,
            weight: u32,
        },
    }

    /// A storage item that this pallet contains. This will be part of the state root trie
    /// of the blockchain.
    #[pallet::storage]
    pub type Value<T> = StorageValue<Value = u128>;

    /// All *dispatchable* call functions (aka. transactions) are attached to `Pallet` in a
    /// `impl` block.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// This will be callable by external users, and has two u32s as a parameter.
        pub fn inventory_insertion(
            origin: OriginFor<T>,
            sku: BoundedVec<u8, ConstU32<16>>,
            qty: u128,
            weight: u32,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Request storage of the new item
            Self::do_request_storage(who, sku, qty, weight)?;

            // Emit an event detailing that a new randomness is available
            Self::deposit_event(Event::AddNewItem {
                sender: who,
                sku,
                qty,
                weight,
            });

            Ok(())
        }
    }
}
