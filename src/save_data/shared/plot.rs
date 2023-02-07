use anyhow::Result;
use bitvec::prelude::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Deref, DerefMut)]
pub struct BitVec(bitvec::vec::BitVec<u32, Lsb0>);

impl<'de> Deserialize<'de> for BitVec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bitfields: Vec<u32> = Deserialize::deserialize(deserializer)?;
        let bitvec = bitvec::vec::BitVec::from_vec(bitfields);
        Ok(BitVec(bitvec))
    }
}

impl serde::Serialize for BitVec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let bitfields = self.0.clone().into_vec();
        serializer.collect_seq(bitfields)
    }
}

#[derive(Deserialize, Serialize)]
pub struct PlotTable {
    pub booleans: BitVec,
    pub integers: Vec<i32>,
    pub floats: Vec<f32>,
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
    history: Vec<i32>,
}

#[derive(Deserialize, Serialize, Default)]
pub struct PlotCodex {
    pages: Vec<PlotCodexPage>,
}

#[derive(Deserialize, Serialize, Default)]
pub struct PlotCodexPage {
    page: i32,
    is_new: bool,
}
