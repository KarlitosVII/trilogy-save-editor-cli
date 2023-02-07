use serde::{Deserialize, Serialize};

use super::player::Power;
use crate::save_data::shared::player::WeaponLoadout;

#[derive(Deserialize, Serialize, Default)]
pub struct Henchman {
    tag: String,
    powers: Vec<Power>,
    character_level: i32,
    talent_points: i32,
    weapon_loadout: WeaponLoadout,
    mapped_power: String,
}
