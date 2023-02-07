use serde::{Deserialize, Serialize};

use crate::{
    save_data::shared::plot::{BitVec, PlotCodex},
    IndexMap,
};

#[derive(Deserialize, Serialize)]
pub struct PlotTable {
    pub booleans: BitVec,
    pub integers: IndexMap<i32, i32>,
    pub floats: IndexMap<i32, f32>,
}

#[derive(Deserialize, Serialize)]
pub struct Journal {
    quest_progress_counter: i32,
    quest_progress: Vec<PlotQuest>,
    quest_ids: Vec<i32>,
}

#[derive(Deserialize, Serialize)]
pub struct Codex {
    codex_entries: Vec<PlotCodex>,
    codex_ids: Vec<i32>,
}

#[derive(Deserialize, Serialize, Default)]
pub struct PlotQuest {
    quest_counter: i32,
    quest_updated: bool,
    active_goal: i32,
    history: Vec<i32>,
}
