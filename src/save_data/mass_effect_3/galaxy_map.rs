use serde::{Deserialize, Serialize};

use crate::save_data::shared::Vector2D;

#[derive(Deserialize, Serialize)]
pub struct GalaxyMap {
    planets: Vec<Planet>,
    systems: Vec<System>,
}

#[derive(Deserialize, Serialize, Default)]
pub struct Planet {
    id: i32,
    visited: bool,
    probes: Vec<Vector2D>,
    show_as_scanned: bool,
}

#[derive(Deserialize, Serialize, Default)]
pub struct System {
    id: i32,
    reaper_alert_level: f32,
    reaper_detected: bool,
}
