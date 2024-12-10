# Inventory Management Pallet

This pallet provides the core functionality for managing inventory within a Polkadot-based blockchain project. It's the preliminary implementation of an ERP blockchain system, focusing on inventory management for the time being. The pallet
allows for adding new inventory items, organizing them by SKUs, and associating detailed metadata such as lot numbers,
serial numbers, and product types.

## Features

- **Storage for SKUs and Metadata**: The pallet stores SKUs and their associated metadata (e.g., lot numbers, inventory type, product type, quantity, and more). SKUs are stored along with Item structs that include the details of each inventory entry.

- **Transaction Calls**: The pallet allows authorized users to insert new inventory records into the blockchain.

- **Events**: The pallet emits events when inventory items are added. These events are stored on-chain and can be queried by off-chain tools or external applications.

## Functions

### Storage

The primary storage structure in this pallet is defined as:

```rust
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
```

- **SKU**: Each SKU is tied to an AccountId (the user who inserted it) and can store multiple lots.
- **Item**: Each SKU contains a vector of Item structs, which holds the details about individual inventory entries.

### Dispatchable Calls

`inventory_insertion`
Allows a user to add an inventory item associated with an SKU.

```rust
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
) -> DispatchResult;
```

#### Parameters:

- **sku**: A unique identifier for the product.
- **moved_by**: Account ID responsible for moving or handling the inventory.
- **lot_number**: Identifier for the batch or lot of the product.
- **serial_number**: Unique serial number associated with the product.
- **abc_code**: Classification code for the item (typically for inventory categorization).
- **inventory_type**: The type of inventory (e.g., raw materials, finished goods).
- **product_type**: The type of product (e.g., electronics, furniture).
- **qty**: Quantity of the product in the inventory.
- **weight_lbs**: The weight of the product.
- **cycle_count**: The cycle count for the inventory item (used in periodic checks).
- **shelf_life**: The shelf life of the product.

`Event::AddNewItem`
This event is emitted when a new item is successfully added to the inventory.

```rust
Event::AddNewItem {
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
}
```

### Error Handling

The pallet provides error messages to handle issues such as:

- **ConversionFailed**: When a data conversion fails.
- **InvalidSkuLength**: When the SKU length is invalid (e.g., too short or too long).

### To-Do List

- **Storage for SKUs and Metadata**: Add more detailed metadata management for SKUs.
- **Add Tests**: Write unit and integration tests to ensure the correctness of inventory storage and event emission.
- **Add Deletion Functionality**: Implement a dispatchable call to remove SKUs and their associated data.
- **Optimize Storage**: Consider merging storage fields or using more efficient data structures for performance.
- **New Modules**: Current WIP on Orders, Sales and Packaging

### Prerequisites

Before using this pallet, ensure that your runtime includes the following dependencies:

- **pallet_timestamp**: To manage timestamps for inventory items (e.g., for shelf life).
- **frame_system**: Core system pallet for handling account and event storage.

frame-omni-bencher v1 benchmark pallet \
 --runtime target/production/wbuild/erp-blockchain-runtime/erp_blockchain_runtime.compact.compressed.wasm \
 --pallet "pallet-inventory" \
 --extrinsic "\*" \
 --genesis-builder-preset=production \
 --output ./pallets/inventory/src/weights.rs \
 --template .maintain/frame-weight-template.hbs
