use serde::{Deserialize, Serialize};

use super::player::{Power, Weapon, WeaponMod};
use crate::save_data::shared::player::WeaponLoadout;

#[derive(Deserialize, Serialize, Default)]
pub struct Henchman {
    tag: String,
    powers: Vec<Power>,
    character_level: i32,
    talent_points: i32,
    weapon_loadout: WeaponLoadout,
    mapped_power: String,
    weapon_mods: Vec<WeaponMod>,
    grenades: i32,
    weapons: Vec<Weapon>,
}
