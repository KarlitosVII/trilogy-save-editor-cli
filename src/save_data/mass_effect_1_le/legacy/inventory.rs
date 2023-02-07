use serde::{Deserialize, Serialize};

use super::{BaseObject, OptionObjectProxy};
use crate::save_data::mass_effect_1_le::player::ItemLevel;

#[derive(Deserialize, Serialize)]
pub struct Shop {
    last_player_level: i32,
    is_initialized: bool,
    inventory: Vec<OptionObjectProxy>,
}

#[derive(Deserialize, Serialize)]
pub struct Inventory {
    items: Vec<BaseObject>,
    plot_items: Vec<PlotItem>,
    credits: i32,
    grenades: i32,
    medigel: f32,
    omnigel: f32,
}

#[derive(Deserialize, Serialize, Default)]
pub struct PlotItem {
    localized_name: i32,
    localized_desc: i32,
    export_id: i32,
    base_price: i32,
    shop_gui_image_id: i32,
    plot_conditional_id: i32,
}

#[derive(Deserialize, Serialize)]
pub struct Item {
    item_id: i32,
    item_level: ItemLevel,
    manufacturer_id: i32,
    plot_conditional_id: i32,
    slot_specs: Vec<ModdableSlotSpec>,
}

#[derive(Deserialize, Serialize, Default)]
pub struct ModdableSlotSpec {
    type_id: i32,
    mods: Vec<OptionObjectProxy>,
}

#[derive(Deserialize, Serialize)]
pub struct ItemMod {
    item_id: i32,
    item_level: ItemLevel,
    manufacturer_id: i32,
    plot_conditional_id: i32,
    type_id: i32,
}
