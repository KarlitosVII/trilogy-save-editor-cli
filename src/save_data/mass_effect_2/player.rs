use serde::{Deserialize, Serialize};

use crate::save_data::shared::{
    appearance::Appearance,
    player::{Notoriety, Origin, WeaponLoadout},
};

#[derive(Deserialize, Serialize)]
pub struct Player {
    pub is_female: bool,
    pub class_name: String,
    pub level: i32,
    pub current_xp: f32,
    pub first_name: String,
    localized_last_name: i32,
    pub origin: Origin,
    pub notoriety: Notoriety,
    pub talent_points: i32,
    mapped_power_1: String,
    mapped_power_2: String,
    mapped_power_3: String,
    pub appearance: Appearance,
    pub powers: Vec<Power>,
    weapons: Vec<Weapon>,
    weapons_loadout: WeaponLoadout,
    hotkeys: Vec<Hotkey>,
    pub credits: i32,
    pub medigel: i32,
    pub eezo: i32,
    pub iridium: i32,
    pub palladium: i32,
    pub platinum: i32,
    pub probes: i32,
    pub current_fuel: f32,
    pub face_code: String,
    localized_class_name: i32,
}

#[derive(Deserialize, Serialize, Default)]
pub struct Power {
    pub name: String,
    rank: f32,
    pub power_class_name: String,
    wheel_display_index: i32,
}

#[derive(Deserialize, Serialize, Default)]
struct Weapon {
    class_name: String,
    ammo_used_count: i32,
    ammo_total: i32,
    current_weapon: bool,
    last_weapon: bool,
    ammo_power_name: String,
}

#[derive(Deserialize, Serialize, Default)]
pub struct Hotkey {
    pawn_name: String,
    power_id: i32,
}
