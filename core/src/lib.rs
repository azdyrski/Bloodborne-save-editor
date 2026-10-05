use std::cell::RefCell;

use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use wasm_bindgen::prelude::*;

pub mod data_handling;

use data_handling::{
    appearance,
    article::Article,
    enums::{ArticleType, Location, SlotShape, UpgradeType},
    save::SaveData,
    upgrades::Upgrade,
};

thread_local! {
    static SAVE: RefCell<Option<SaveData>> = RefCell::new(None);
}

#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

fn with_save<R>(f: impl FnOnce(&mut SaveData) -> Result<R, String>) -> Result<R, String> {
    SAVE.with(|cell| {
        let mut guard = cell.borrow_mut();
        let save = guard.as_mut().ok_or("No save loaded")?;
        f(save)
    })
}

fn arg<T: DeserializeOwned>(args: &Value, key: &str) -> Result<T, String> {
    serde_json::from_value(args[key].clone()).map_err(|e| format!("Invalid argument '{key}': {e}"))
}

fn to_json(save: &SaveData) -> Result<Value, String> {
    serde_json::to_value(save).map_err(|e| e.to_string())
}

fn location(is_storage: bool) -> Location {
    if is_storage {
        Location::Storage
    } else {
        Location::Inventory
    }
}

fn resolve_upgrade(save: &mut SaveData, info: &Value) -> Result<*mut Upgrade, String> {
    let loc = location(arg(info, "isStorage")?);

    let upgrade = if let Some(equipped) = info.get("equipped") {
        let article_type: ArticleType = arg(equipped, "articleType")?;
        let article_index: usize = arg(equipped, "articleIndex")?;
        let slot_index: usize = arg(equipped, "slotIndex")?;

        save.get_equipped_upgrade_mut(loc, article_type, article_index, slot_index)
            .map(|u| u as *mut Upgrade)
    } else {
        let upgrade_type: UpgradeType = arg(info, "upgradeType")?;
        let upgrade_index: usize = arg(info, "upgradeIndex")?;

        save.get_upgrade_mut(loc, upgrade_type, upgrade_index)
            .map(|u| u as *mut Upgrade)
    };

    upgrade.ok_or_else(|| "Upgrade not found".to_string())
}

/// Parses a save file and keeps it as the active save. Returns the save as JSON.
#[wasm_bindgen]
pub fn load_save(bytes: &[u8]) -> Result<String, String> {
    let save = SaveData::from_bytes(bytes.to_vec())
        .map_err(|_| "Failed to load file, make sure its a decrypted character.".to_string())?;
    let json = serde_json::to_string(&save).map_err(|e| e.to_string())?;
    SAVE.with(|cell| *cell.borrow_mut() = Some(save));
    Ok(json)
}

#[wasm_bindgen]
pub fn get_save_bytes() -> Result<Vec<u8>, String> {
    with_save(|save| Ok(save.file.bytes.clone()))
}

#[wasm_bindgen]
pub fn export_appearance_bytes() -> Result<Vec<u8>, String> {
    with_save(|save| Ok(appearance::export_bytes(&save.file)))
}

#[wasm_bindgen]
pub fn import_appearance_bytes(bytes: &[u8]) -> Result<String, String> {
    with_save(|save| {
        appearance::import_bytes(&mut save.file, bytes)
            .map_err(|_| "The imported file is not a face".to_string())?;
        Ok("Successfully imported".to_string())
    })
}

#[wasm_bindgen]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Runs a command by name with a JSON object of camelCase arguments, returning JSON.
#[wasm_bindgen]
pub fn invoke(cmd: &str, args_json: &str) -> Result<String, String> {
    let args: Value = serde_json::from_str(args_json).map_err(|e| e.to_string())?;
    let result = dispatch(cmd, &args)?;
    serde_json::to_string(&result).map_err(|e| e.to_string())
}

fn dispatch(cmd: &str, args: &Value) -> Result<Value, String> {
    match cmd {
        "return_weapons" => serde_json::from_str(include_str!("../resources/weapons.json"))
            .map_err(|e| e.to_string()),
        "return_armors" => serde_json::from_str(include_str!("../resources/armors.json"))
            .map_err(|e| e.to_string()),
        "return_items" => serde_json::from_str(include_str!("../resources/items.json"))
            .map_err(|e| e.to_string()),
        "return_gem_effects" | "return_rune_effects" => {
            let upgrades: Value = serde_json::from_str(include_str!("../resources/upgrades.json"))
                .map_err(|e| e.to_string())?;
            let key = if cmd == "return_gem_effects" {
                "gemEffects"
            } else {
                "runeEffects"
            };
            Ok(upgrades[key].clone())
        }
        "get_version" => Ok(json!(get_version())),

        "set_flag" => with_save(|save| {
            save.file.set_flag(arg(args, "offset")?, arg(args, "newValue")?);
            Ok(Value::Null)
        }),
        "apply_mask" => with_save(|save| {
            save.file.apply_mask(arg(args, "offset")?, arg(args, "mask")?);
            Ok(Value::Null)
        }),
        "get_isz" => with_save(|save| Ok(json!(save.file.get_isz()))),
        "fix_isz" => with_save(|save| Ok(json!(save.file.fix_isz()))),
        "get_playtime" => with_save(|save| Ok(json!(save.file.get_playtime()))),
        "set_playtime" => with_save(|save| {
            let playtime: [u8; 4] = arg(args, "newPlaytime")?;
            save.file.set_playtime(playtime);
            Ok(Value::Null)
        }),
        "edit_stat" => with_save(|save| {
            save.file.edit(
                arg(args, "relOffset")?,
                arg(args, "length")?,
                arg(args, "times")?,
                arg(args, "value")?,
            );
            Ok(Value::Null)
        }),
        "set_username" => with_save(|save| {
            let name: String = arg(args, "newUsername")?;
            save.username
                .set(&mut save.file, name)
                .map_err(|_| "Failed to change name".to_string())?;
            Ok(json!("Successfully changed name"))
        }),
        "edit_coordinates" => with_save(|save| {
            let (x, y, z): (f32, f32, f32) = (arg(args, "x")?, arg(args, "y")?, arg(args, "z")?);
            save.position.coordinates.edit(&mut save.file, x, y, z);
            Ok(Value::Null)
        }),
        "teleport" => with_save(|save| {
            let (x, y, z): (f32, f32, f32) = (arg(args, "x")?, arg(args, "y")?, arg(args, "z")?);
            let map_id: Vec<u8> = arg(args, "mapId")?;
            if map_id.len() < 2 {
                return Err("Invalid map id".to_string());
            }
            let le_map = [0, 0, map_id[1], map_id[0]];

            for (i, j) in (0x04..0x08).enumerate() {
                save.file.bytes[j] = le_map[i];
            }

            save.position.coordinates.edit(&mut save.file, x, y, z);
            Ok(Value::Null)
        }),

        "edit_quantity" => with_save(|save| {
            let number: u8 = arg(args, "number")?;
            let id: u32 = arg(args, "id")?;
            let value: u32 = arg(args, "value")?;
            let is_storage: bool = arg(args, "isStorage")?;

            let inventory = if is_storage {
                &mut save.storage
            } else {
                &mut save.inventory
            };
            inventory
                .edit_item(&mut save.file, number, id, value, is_storage)
                .map_err(|e| e.to_string())?;
            to_json(save)
        }),
        "add_item" => with_save(|save| {
            let id: u32 = arg(args, "id")?;
            let quantity: u32 = arg(args, "quantity")?;
            let is_storage: bool = arg(args, "isStorage")?;

            let inventory = if is_storage {
                &mut save.storage
            } else {
                &mut save.inventory
            };
            inventory
                .add_item(&mut save.file, id, quantity, is_storage)
                .map_err(|_| "Failed to add the item".to_string())?;
            to_json(save)
        }),
        "transform_item" => with_save(|save| {
            let index: usize = arg(args, "index")?;
            let id: u32 = arg(args, "id")?;
            let new_id: u32 = arg(args, "newId")?;
            let article_type: ArticleType = arg(args, "articleType")?;
            let is_storage: bool = arg(args, "isStorage")?;

            let inventory = if is_storage {
                &mut save.storage
            } else {
                &mut save.inventory
            };
            let item = inventory
                .articles
                .get_mut(&article_type)
                .and_then(|c| c.iter_mut().find(|x| x.id == id && x.index == index))
                .ok_or("Item not found")?;

            let old_type = item.article_type;
            item.transform(&mut save.file, new_id, is_storage)
                .map_err(|e| e.to_string())?;

            // Move the item to its new category if the type changed
            if item.article_type != old_type {
                let moved_item = item.clone();

                if let Some(old_category) = save.inventory.articles.get_mut(&old_type) {
                    old_category.retain(|x| x.index != index);
                }

                save.inventory
                    .articles
                    .entry(moved_item.article_type)
                    .or_insert_with(Vec::new)
                    .push(moved_item);
            }

            to_json(save)
        }),
        "edit_effect" => with_save(|save| {
            let new_effect_id: u32 = arg(args, "newEffectId")?;
            let index: usize = arg(args, "index")?;
            let upgrade = resolve_upgrade(save, &args["info"])?;

            // SAFETY: the pointer comes from `save` and `save.file` is a disjoint field.
            unsafe {
                (*upgrade)
                    .change_effect(&mut save.file, new_effect_id, index)
                    .map_err(|_| "Failed to edit the upgrade's effect".to_string())?;
            }
            to_json(save)
        }),
        "edit_shape" => with_save(|save| {
            let new_shape: String = arg(args, "newShape")?;
            let upgrade = resolve_upgrade(save, &args["info"])?;

            // SAFETY: the pointer comes from `save` and `save.file` is a disjoint field.
            unsafe {
                (*upgrade)
                    .change_shape(&mut save.file, new_shape)
                    .map_err(|_| "Failed to edit the upgrade's shape".to_string())?;
            }
            to_json(save)
        }),
        "edit_slot" => with_save(|save| {
            let loc = location(arg(args, "isStorage")?);
            let article_type: ArticleType = arg(args, "articleType")?;
            let article_index: usize = arg(args, "articleIndex")?;
            let slot_index: usize = arg(args, "slotIndex")?;
            let new_shape: SlotShape = arg(args, "newShape")?;

            let article = save
                .get_article_mut(loc, article_type, article_index)
                .map(|a| a as *mut Article)
                .ok_or("Article not found")?;

            // SAFETY: the pointer comes from `save` and `save.file` is a disjoint field.
            unsafe {
                (*article)
                    .change_slot_shape(&mut save.file, slot_index, new_shape)
                    .map_err(|e| e.to_string())?;
            }
            to_json(save)
        }),
        "equip_gem" => with_save(|save| {
            let upgrade_index: usize = arg(args, "upgradeIndex")?;
            let article_type: ArticleType = arg(args, "articleType")?;
            let article_index: usize = arg(args, "articleIndex")?;
            let slot_index: usize = arg(args, "slotIndex")?;
            let is_storage: bool = arg(args, "isStorage")?;

            let inventory = if is_storage {
                &mut save.storage
            } else {
                &mut save.inventory
            };
            inventory
                .equip_gem(
                    &mut save.file,
                    upgrade_index,
                    article_type,
                    article_index,
                    slot_index,
                    is_storage,
                )
                .map_err(|e| e.to_string())?;
            to_json(save)
        }),
        "unequip_gem" => with_save(|save| {
            let article_type: ArticleType = arg(args, "articleType")?;
            let article_index: usize = arg(args, "articleIndex")?;
            let slot_index: usize = arg(args, "slotIndex")?;
            let is_storage: bool = arg(args, "isStorage")?;

            let inventory = if is_storage {
                &mut save.storage
            } else {
                &mut save.inventory
            };
            inventory
                .unequip_gem(
                    &mut save.file,
                    article_type,
                    article_index,
                    slot_index,
                    is_storage,
                )
                .map_err(|e| e.to_string())?;
            to_json(save)
        }),
        "change_weapon_level" => with_save(|save| {
            let article_type: ArticleType = arg(args, "articleType")?;
            let article_index: usize = arg(args, "articleIndex")?;
            let slot_index: usize = arg(args, "slotIndex")?;
            let is_storage: bool = arg(args, "isStorage")?;
            let level: u8 = arg(args, "level")?;

            let inventory = if is_storage {
                &mut save.storage
            } else {
                &mut save.inventory
            };
            let weapon = inventory
                .change_weapon_level(
                    &mut save.file,
                    article_type,
                    article_index,
                    slot_index,
                    is_storage,
                    level,
                )
                .map_err(|e| e.to_string())?;

            Ok(json!({ "save": to_json(save)?, "weapon": weapon }))
        }),
        _ => Err(format!("Unknown command: {cmd}")),
    }
}
