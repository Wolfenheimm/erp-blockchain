use crate::Config;
use crate::Error;
use crate::{pallet::Pallet, types::*};
use crate::{AdjustInventory, GlobalInventory, Inventory, Materials, Recipes, ScrapInventory};
use crate::{Event, InventoryLocale};
use codec::{Encode, MaxEncodedLen};
use frame_support::pallet_prelude::DispatchError;
use frame_support::sp_runtime::DispatchResult;
use frame_support::BoundedBTreeMap;

impl<T: Config> Pallet<T> {
    /// Insert a new item into inventory
    pub fn do_inventory_insertion(who: &T::AccountId, item: Item) -> DispatchResult {
        // Ensure SKU length does not exceed 16
        let sku_encoded_len = item.sku.encode().len();
        let max_encoded_len = Sku::max_encoded_len();

        if sku_encoded_len > max_encoded_len {
            return Err(Error::<T>::InvalidSkuLength.into());
        }

        // Insert the updated BoundedBTreeMap back into storage
        <Inventory<T>>::insert((who, item.sku.clone(), item.serial_number), item.clone());

        // Update the global inventory quantity
        <GlobalInventory<T>>::try_mutate(item.sku.clone(), |qty| -> DispatchResult {
            if let Some(existing_qty) = qty {
                *existing_qty = existing_qty.saturating_add(item.qty); // Safely add quantity
            } else {
                *qty = Some(item.qty); // Initialize the quantity
            }
            Ok(())
        })?;

        // Fist, check if the location already exists in the InventoryLocale storage
        // If it does, append the item to the existing Vec<Item> and insert it back into storage
        // If it does not, create a new Vec<Item> and insert it into the InventoryLocale storage
        <InventoryLocale<T>>::try_mutate_exists(
            item.location.clone(),
            |inventory| -> DispatchResult {
                match inventory {
                    Some(ref mut map) => {
                        // Insert the item using its serial number as the key
                        map.try_insert(item.serial_number.clone(), item.clone())
                            .map_err(|_| Error::<T>::InventoryFull)?;
                    }
                    None => {
                        // Create a new BoundedBTreeMap and insert the item
                        let mut new_map = BoundedBTreeMap::default();
                        new_map
                            .try_insert(item.serial_number.clone(), item.clone())
                            .map_err(|_| Error::<T>::InventoryFull)?;
                        *inventory = Some(new_map);
                    }
                }
                Ok(())
            },
        )?;

        // Emit the insertion
        Self::deposit_event(Event::AddNewItem {
            sender: who.clone(),
            item,
        });

        Ok(())
    }

    /// Scrap an item from inventory
    ///
    /// Removes the item from the inventory and moves it to the scrap inventory
    pub fn do_inventory_scrap(
        who: &T::AccountId,
        sku: Sku,
        serial_number: SerialNumber,
        scrap_details: ScrapDetails,
    ) -> DispatchResult {
        // Fetch the item to be scrapped
        let item = <Inventory<T>>::take((who, sku.clone(), serial_number))
            .ok_or(Error::<T>::InventoryNotFound)?;

        let scrap_item = ScrapItem {
            item: item.clone(),
            details: scrap_details,
        };

        // Scrap the item by sending it to the ScrapInventory storage
        <ScrapInventory<T>>::insert((who, sku.clone(), serial_number), scrap_item);
        <GlobalInventory<T>>::try_mutate_exists(sku, |qty| -> DispatchResult {
            if let Some(current_qty) = qty {
                if *current_qty >= item.qty {
                    *current_qty -= item.qty;
                    if *current_qty == Qty::default() {
                        *qty = None; // Remove entry if quantity becomes 0
                    }
                    Ok(())
                } else {
                    Err(Error::<T>::InsufficientInventory.into())
                }
            } else {
                Err(Error::<T>::InventoryNotFound.into())
            }
        })?;
        // Remove the item from InventoryLocale
        <InventoryLocale<T>>::try_mutate_exists(
            item.location.clone(),
            |location_items| -> DispatchResult {
                if let Some(ref mut map) = location_items {
                    map.remove(&serial_number); // Remove the item by its SerialNumber
                } else {
                    return Err(Error::<T>::LocationNotFound.into());
                }
                Ok(())
            },
        )?;

        Ok(())
    }

    /// Move an item from one location to another
    pub fn do_inventory_move(
        who: &T::AccountId,
        mut item: Item,
        moved_by: Employee,
        adjust_details: AdjustDetails,
    ) -> DispatchResult {
        // Get the enum variant of the AdjustDetails
        match adjust_details {
            AdjustDetails::Location {
                ref new_location,
                ref original_location,
                ..
            } => {
                // Update the item's fields
                item.moved_by = moved_by.clone();
                item.location = new_location.clone();

                // Remove the item from the old location
                <InventoryLocale<T>>::try_mutate_exists(
                    original_location.clone(),
                    |location_items| -> DispatchResult {
                        if let Some(ref mut map) = location_items {
                            map.remove(&item.serial_number); // Remove the item by its SerialNumber
                            Ok(())
                        } else {
                            Err(Error::<T>::LocationNotFound.into())
                        }
                    },
                )?;

                // Insert the item into the new location
                <InventoryLocale<T>>::try_mutate(
                    new_location.clone(),
                    |location_items| -> Result<(), DispatchError> {
                        if let Some(ref mut map) = location_items {
                            map.try_insert(item.serial_number.clone(), item.clone())
                                .map_err(|_| DispatchError::from(Error::<T>::InventoryFull))?;
                        } else {
                            let mut new_map = BoundedBTreeMap::default();
                            new_map
                                .try_insert(item.serial_number.clone(), item.clone())
                                .map_err(|_| DispatchError::from(Error::<T>::InventoryFull))?;
                            *location_items = Some(new_map);
                        }
                        Ok(())
                    },
                )?;

                // Insert the updated item back into storage
                <Inventory<T>>::mutate(
                    (who, item.sku.clone(), item.serial_number),
                    |stored_item| {
                        *stored_item = Some(item.clone());
                    },
                );

                // Insert the adjustment for auditing purposes
                <AdjustInventory<T>>::insert(
                    (who, item.sku.clone(), item.serial_number),
                    AdjustItem {
                        issuer: moved_by.clone(),
                        item: item.clone(),
                        adjust_details: adjust_details.clone(),
                    },
                );

                // Emit the move
                Self::deposit_event(Event::MoveItem {
                    sender: who.clone(),
                    item: item.clone(),
                    moved_by,
                    adjust_details,
                });
            }
            // Fail if the AdjustDetails variant is not Location
            _ => return Err(Error::<T>::InvalidAdjustDetails.into()),
        }

        Ok(())
    }

    /// Adjust the quantity of an item in inventory
    pub fn do_inventory_adjust(
        who: &T::AccountId,
        issuer: Employee,
        mut item: Item,
        adjust_details: AdjustDetails,
    ) -> DispatchResult {
        match adjust_details {
            AdjustDetails::Quantity {
                ref original_qty,
                ref new_qty,
                ..
            } => {
                item.qty = *new_qty;
                // Update the item in storage
                <Inventory<T>>::mutate(
                    (who, item.sku.clone(), item.serial_number),
                    |stored_item| {
                        *stored_item = Some(item.clone());
                    },
                );
                // Update the global inventory quantity
                <GlobalInventory<T>>::try_mutate_exists(
                    item.sku.clone(),
                    |qty| -> DispatchResult {
                        if let Some(current_qty) = qty {
                            // Calculate the difference
                            let diff = *new_qty as i64 - *original_qty as i64;

                            // Update the global quantity
                            if diff > 0 {
                                *current_qty = current_qty.saturating_add(diff as u32);
                            } else {
                                let abs_diff = diff.abs() as u32;
                                if *current_qty >= abs_diff {
                                    *current_qty -= abs_diff;
                                } else {
                                    // If underflow would occur, handle error gracefully
                                    return Err(Error::<T>::InsufficientInventory.into());
                                }
                            }
                            Ok(())
                        } else {
                            Err(Error::<T>::InventoryNotFound.into())
                        }
                    },
                )?;

                // Update the items in the InventoryLocale storage
                <InventoryLocale<T>>::try_mutate_exists(
                    item.location.clone(),
                    |location_items| -> DispatchResult {
                        if let Some(ref mut map) = location_items {
                            map.try_insert(item.serial_number.clone(), item.clone())
                                .map_err(|_| Error::<T>::InventoryFull)?;
                        } else {
                            return Err(Error::<T>::LocationNotFound.into());
                        }
                        Ok(())
                    },
                )?;

                // Insert the adjustment for auditing purposes
                <AdjustInventory<T>>::insert(
                    (who, item.sku.clone(), item.serial_number),
                    AdjustItem {
                        issuer,
                        item: item.clone(),
                        adjust_details: adjust_details.clone(),
                    },
                );

                // Emit the adjustment
                Self::deposit_event(Event::AdjustItem {
                    sender: who.clone(),
                    item: item.clone(),
                    issuer: item.moved_by,
                    adjust_details,
                });
            }
            // Fail if the AdjustDetails variant is not Location
            _ => return Err(Error::<T>::InvalidAdjustDetails.into()),
        }

        Ok(())
    }

    /// Insert a new recipe into storage
    pub fn do_insert_recipe(recipe: Recipe) -> DispatchResult {
        // Ensure SKU length does not exceed 16
        let sku_encoded_len = recipe.sku.encode().len();
        let max_encoded_len = Sku::max_encoded_len();

        if sku_encoded_len > max_encoded_len {
            return Err(Error::<T>::InvalidSkuLength.into());
        }

        // Insert the updated BoundedBTreeMap back into storage
        <Recipes<T>>::insert(recipe.sku.clone(), recipe);

        Ok(())
    }

    /// Insert a new material into storage
    pub fn do_insert_material(material: Material) -> DispatchResult {
        // Check if the material already exists
        if <Materials<T>>::contains_key(&material.sku) {
            return Err(Error::<T>::MaterialAlreadyExists.into());
        }

        // Insert the material if it does not already exist
        <Materials<T>>::insert(material.sku.clone(), material);

        Ok(())
    }

    /// Delete a material from storage
    pub fn do_delete_material(sku: Sku) -> DispatchResult {
        // Check if the material exists
        if !<Materials<T>>::contains_key(&sku) {
            return Err(Error::<T>::MaterialNotFound.into());
        }

        // Remove the material
        <Materials<T>>::remove(&sku);

        Ok(())
    }

    /// Update a material in storage
    pub fn do_update_material(material: Material) -> DispatchResult {
        // Check if the material exists
        <Materials<T>>::mutate_exists(
            material.sku.clone(),
            |existing_material| -> DispatchResult {
                if let Some(ref mut current_material) = existing_material {
                    // Update the existing material in place
                    *current_material = material;
                    Ok(())
                } else {
                    // Return an error if the material does not exist
                    Err(Error::<T>::MaterialNotFound.into())
                }
            },
        )?;

        Ok(())
    }
}
