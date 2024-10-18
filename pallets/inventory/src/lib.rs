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
    pub use crate::types::{Item, Lot, AbcCode, InventoryType, ProductType, Sku, MovedByAccount};
    use frame::prelude::*;
    use pallet_timestamp::{self as timestamp};
    use sp_std::vec::Vec;
    use crate::types::{CycleCount, LotNumber, Qty, SerialNumber, ShelfLife, Weight};

    #[pallet::config]
    pub trait Config: frame_system::Config + timestamp::Config {
        type RuntimeEvent: IsType<<Self as frame_system::Config>::RuntimeEvent> + From<Event<Self>>;
    }

    #[pallet::error]
    pub enum Error<T> {
        ConversionFailed,
        InvalidSkuLength,
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::event]
    #[pallet::generate_deposit(pub(crate) fn deposit_event)]
    pub enum Event<T: Config> {
        AddNewItem {
            sender: T::AccountId,
            sku: Sku,
            moved_by: MovedByAccount,
            lot_number: LotNumber,
            serial_number: SerialNumber,
            abc_code: AbcCode,
            inventory_type: InventoryType,
            product_type: ProductType,
            qty: Qty,
            weight: Weight,
            cycle_count: CycleCount,
            shelf_life: ShelfLife,
        },
    }

    #[pallet::storage]
    pub type Value<T: Config> = StorageNMap<
        _,
        (
            NMapKey<Blake2_128Concat, T::AccountId>,
            NMapKey<Blake2_128Concat, Sku>,
        ),
        Vec<(Lot, Vec<Item<T>>)>,
        OptionQuery,
    >;


    #[pallet::call]
    impl<T: Config> Pallet<T> {
        pub fn inventory_insertion(
            origin: OriginFor<T>,
            sku: Sku,
            moved_by: MovedByAccount,
            lot_number: LotNumber,
            serial_number: SerialNumber,
            abc_code: AbcCode,
            inventory_type: InventoryType,
            product_type: ProductType,
            qty: Qty,
            weight: Weight,
            cycle_count: CycleCount,
            shelf_life: ShelfLife,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            Self::do_inventory_insertion(&who, &sku, &moved_by, &lot_number, &serial_number, &abc_code, &inventory_type, &product_type, &qty, &weight, &cycle_count, &shelf_life)?;

            Self::deposit_event(Event::AddNewItem {
                sender: who,
                sku,
                moved_by,
                lot_number,
                serial_number,
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
