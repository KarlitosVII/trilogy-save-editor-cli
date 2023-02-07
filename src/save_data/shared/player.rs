use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Copy, Clone)]
pub enum Origin {
    None,
    Spacer,
    Colonist,
    Earthborn,
}

#[derive(Deserialize, Serialize, Copy, Clone)]
pub enum Notoriety {
    None,
    Survivor,
    Warhero,
    Ruthless,
}

#[derive(Deserialize, Serialize, Default)]
pub struct WeaponLoadout {
    assault_rifle: String,
    shotgun: String,
    sniper_rifle: String,
    submachine_gun: String,
    pistol: String,
    heavy_weapon: String,
}
