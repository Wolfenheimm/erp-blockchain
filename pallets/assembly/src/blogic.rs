use crate::pallet::{AssembledProducts, Config, Error, WorkOrders};
use crate::{pallet::Pallet, types::*, Event};
use frame_support::ensure;
use frame_support::sp_runtime::DispatchResult;
use frame_support::BoundedVec;
use pallet_inventory::types::{AdjustDetails, Item, Location, SerialNumber};
use pallet_inventory::{GlobalInventory, Inventory, InventoryLocale};
use scale_info::prelude::vec::Vec;

impl<T: Config> Pallet<T> {
    /// Assemble a product from a given Work Order
    ///
    /// Must provide the serial number of the assembled product as well as the staging location.
    pub fn do_assemble_product(
        who: &T::AccountId,
        work_order: WorkOrder,
        serial_number: SerialNumber,
        staging_location: Location,
    ) -> DispatchResult {
        // Query the work order and get the details
        let work_order = WorkOrders::<T>::get(work_order.work_order_number)
            .ok_or(Error::<T>::WorkOrderNotFound)?;

        // Create a new assembled product (Basic, for now)
        let assembled_product = Item {
            sku: work_order.recipe.sku.clone(),
            serial_number: serial_number.clone(),
            qty: work_order.recipe.output_quantity,
            // Include other fields as needed for `Item`
            ..Default::default()
        };

        // Query the inventory locale area to get the BOM (this is assumed to be true because the staging area is prepped)
        // TODO: Never assume you will get what is needed. Always check if the staging area is prepped.
        let mut staging_inventory =
            InventoryLocale::<T>::get(staging_location).ok_or(Error::<T>::StagingAreaNotFound)?;

        // Initialize a BoundedVec for the BOM components
        let mut bom = Bom::new();

        // For each recipe component, consume the required quantity from the staging area
        for recipe_item in work_order.recipe.required_components.iter() {
            let recipe_quantity = recipe_item.qty;
            let mut required_quantity = recipe_item.qty;
            log::info!("Recipe Item: {:?}", recipe_item);

            // Iterate through the staging area to find the required component
            for (_, item) in staging_inventory.iter_mut() {
                if recipe_item.sku == item.sku {
                    if item.qty == 0 || required_quantity == 0 {
                        continue;
                    }

                    let original_qty = item.qty;

                    ensure!(
                        item.qty < recipe_quantity,
                        Error::<T>::InsufficientInventory
                    );

                    let mut bom_item = item.clone();

                    // Adjust the quantity of the item according to consumption
                    if item.qty >= recipe_quantity {
                        // Remove the consumed quantity from the staging area
                        item.qty -= recipe_quantity;
                        required_quantity -= recipe_quantity;
                        // Add the consumed quantity to the BOM
                        bom_item.qty = recipe_quantity;
                        bom.materials
                            .try_push(bom_item)
                            .map_err(|_| Error::<T>::BomConstructIssue)?;
                    // There is not enough quantity in the staging area to fulfill the recipe requirement
                    } else if item.qty < recipe_quantity {
                        // There is enough quantity in the staging area to fulfill the leftover requirement
                        if item.qty > required_quantity {
                            // Remove the consumed quantity from the staging area
                            item.qty -= required_quantity;
                            // Add the consumed quantity to the BOM
                            bom_item.qty = required_quantity;
                            bom.materials
                                .try_push(bom_item)
                                .map_err(|_| Error::<T>::BomConstructIssue)?;
                            // Set the required quantity to 0, it has been consumed
                            required_quantity = 0;
                        // There is not enough quantity in the staging area to fulfill the leftover requirement
                        } else {
                            // Add the consumed quantity to the BOM
                            bom.materials
                                .try_push(bom_item)
                                .map_err(|_| Error::<T>::BomConstructIssue)?;
                            // Update the required quantity
                            required_quantity -= item.qty;
                            // Set the item quantity to 0, it has been consumed
                            item.qty = 0;
                        }
                    }

                    log::info!("Staging Item Consumed: {:?}", item.clone());

                    // Update the item's quantity in the staging area
                    pallet_inventory::Pallet::<T>::do_inventory_adjust(
                        who,
                        item.moved_by.clone(),
                        item.clone(),
                        AdjustDetails::Quantity {
                            original_qty,
                            new_qty: item.qty,
                            reason: BoundedVec::try_from("Assemble Product".as_bytes().to_vec())
                                .map_err(|_| Error::<T>::DescriptionTooLong)?,
                        },
                    )?;

                    // Move the item back to the warehouse
                    pallet_inventory::Pallet::<T>::do_inventory_move(
                        who,
                        item.clone(),
                        item.moved_by.clone(),
                        AdjustDetails::Location {
                            original_location: item.location.clone(),
                            new_location: Location::Warehouse,
                            reason: BoundedVec::try_from(
                                "Assembled Product complete, move to warehouse"
                                    .as_bytes()
                                    .to_vec(),
                            )
                            .map_err(|_| Error::<T>::DescriptionTooLong)?,
                        },
                    )?;
                }
            }
        }

        // Insert the assembled product into storage
        AssembledProducts::<T>::insert(
            (who, work_order.recipe.sku.clone(), serial_number.clone()),
            (assembled_product.clone(), bom),
        );

        // Insert the newly created item into inventory
        let _ =
            pallet_inventory::Pallet::<T>::do_inventory_insertion(who, assembled_product.clone());

        Ok(())
    }

    /// Prepare the staging area for a given Work Order
    pub fn do_prepare_staging_area(who: &T::AccountId, work_order: WorkOrder) -> DispatchResult {
        // Check if the work order exists
        let work_order = WorkOrders::<T>::get(work_order.work_order_number)
            .ok_or(Error::<T>::WorkOrderNotFound)?;

        // Create a Bill of Materials (BOM) from the assembly details
        let mut bom = Bom::new();

        // For each component in the BOM, check if the inventory contains enough of it
        for component in work_order.recipe.required_components.iter() {
            let key = (who, component.sku.clone());
            let mut items: Vec<Item> = <Inventory<T>>::iter_prefix(key).map(|(_, v)| v).collect();

            // Ensure we have enough items in GlobalInventory for this component
            let available_qty = <GlobalInventory<T>>::get(&component.sku).unwrap_or_default();

            ensure!(
                available_qty >= component.qty,
                Error::<T>::InsufficientInventory
            );

            // Sort the items by creation date (FIFO)
            items.sort_by_key(|item| item.created_at);

            // Take the required items from inventory, place them into the staging area
            let mut qty = component.qty;
            for item in items.iter_mut() {
                // Skip items with 0 quantity
                if item.qty == 0 {
                    continue;
                }

                // Required quantity has been met
                if qty <= 0 {
                    break;
                }

                // Keep track of the required quantity
                if item.qty > qty {
                    qty = 0;
                } else {
                    qty -= item.qty;
                }

                // Move to staging
                let _ = pallet_inventory::Pallet::<T>::do_inventory_move(
                    who,
                    item.clone(),
                    item.moved_by.clone(),
                    AdjustDetails::Location {
                        original_location: item.location.clone(),
                        new_location: Location::Staging,
                        reason: BoundedVec::try_from("Prepare Staging Area".as_bytes().to_vec())
                            .map_err(|_| Error::<T>::DescriptionTooLong)?,
                    },
                );

                // Add the item to the BOM
                let _ = bom
                    .materials
                    .try_push(item.clone())
                    .map_err(|_| Error::<T>::BomConstructIssue)?;
            }
        }

        // Emit staging has been prepped
        Self::deposit_event(Event::PrepStaging {
            assembler: who.clone(),
            work_order: work_order.clone(),
        });

        Ok(())
    }

    pub fn do_create_work_order(work_order: WorkOrder) -> DispatchResult {
        // Check if the work order already exists
        let work_order_check = WorkOrders::<T>::get(work_order.work_order_number);

        // If the work order already exists, return an error
        if work_order_check.is_some() {
            return Err(Error::<T>::WorkOrderAlreadyExists.into());
        }

        // Insert the work order into storage
        <WorkOrders<T>>::insert(work_order.work_order_number, work_order);

        Ok(())
    }
}
