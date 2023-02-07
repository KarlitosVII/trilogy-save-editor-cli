use serde::{Deserialize, Serialize};

use crate::save_data::shared::Vector2D;

#[derive(Deserialize, Serialize)]
pub struct GalaxyMap {
    planets: Vec<Planet>,
}

#[derive(Deserialize, Serialize, Default)]
pub struct Planet {
    id: i32,
    visited: bool,
    probes: Vec<Vector2D>,
}
