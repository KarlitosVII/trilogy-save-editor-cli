mod art_placeable;
pub mod inventory;
mod pawn;

use self::{art_placeable::*, inventory::*, pawn::*};

use std::fmt;

use serde::{de, Deserialize, Deserializer, Serialize};

use crate::{
    save_data::{
        shared::{Rotator, Vector3D},
        Dummy,
    },
    IndexMap,
};

#[derive(Deserialize, Serialize, Default)]
pub struct Map {
    levels: IndexMap<String, Level>,
    world: Option<BaseObject>,
}

#[derive(Deserialize, Serialize, Default)]
pub struct Level {
    objects: Vec<BaseObject>,
    actors: Vec<String>,
}

#[derive(Serialize)]
pub struct BaseObject {
    pub _class_name: String,
    pub owner_name: String,
    pub owner_class: Option<String>,
    pub _object: Object,
}

impl<'de> Deserialize<'de> for BaseObject {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BaseObjectVisitor;
        impl<'de> de::Visitor<'de> for BaseObjectVisitor {
            type Value = BaseObject;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a BaseObject")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let class_name: String = seq.next_element()?.unwrap();
                let owner_name = seq.next_element()?.unwrap();
                let owner_class = seq.next_element()?.unwrap();
                let object = match class_name.as_str() {
                    "BioPawnBehaviorSaveObject" => Object::PawnBehavior(seq.next_element()?.unwrap()),
                    "BioPawnSaveObject" => Object::Pawn(seq.next_element()?.unwrap()),
                    "BioBaseSquadSaveObject" => Object::BaseSquad(seq.next_element()?.unwrap()),
                    "BioShopSaveObject" => Object::Shop(seq.next_element()?.unwrap()),
                    "BioInventorySaveObject" => Object::Inventory(seq.next_element()?.unwrap()),
                    "BioItemXModdableSaveObject" => Object::Item(seq.next_element()?.unwrap()),
                    "BioItemXModSaveObject" => Object::ItemMod(seq.next_element()?.unwrap()),
                    "BioArtPlaceableBehaviorSaveObject" => Object::ArtPlaceableBehavior(seq.next_element()?.unwrap()),
                    "BioArtPlaceableSaveObject" => Object::ArtPlaceable(seq.next_element()?.unwrap()),
                    "BioVehicleBehaviorSaveObject" => Object::VehicleBehavior(seq.next_element()?.unwrap()),
                    "BioVehicleSaveObject" => Object::Vehicle(seq.next_element()?.unwrap()),
                    "BioWorldInfoSaveObject" => Object::World(seq.next_element()?.unwrap()),
                    _ => unreachable!(),
                };

                Ok(BaseObject {
                    _class_name: class_name,
                    owner_name,
                    owner_class,
                    _object: object,
                })
            }
        }
        deserializer.deserialize_tuple_struct("BaseObject", 4, BaseObjectVisitor)
    }
}

#[derive(Serialize)]
pub enum Object {
    PawnBehavior(Box<PawnBehavior>),
    Pawn(Box<Pawn>),
    BaseSquad(Box<BaseSquad>),
    Shop(Box<Shop>),
    Inventory(Box<Inventory>),
    Item(Box<Item>),
    ItemMod(Box<ItemMod>),
    ArtPlaceableBehavior(Box<ArtPlaceableBehavior>),
    ArtPlaceable(Box<ArtPlaceable>),
    VehicleBehavior(Box<VehicleBehavior>),
    Vehicle(Box<Vehicle>),
    World(Box<World>),
}

#[derive(Deserialize, Serialize, Default)]
pub struct OptionObjectProxy {
    pub proxy: Option<BaseObject>,
}

#[derive(Deserialize, Serialize)]
pub struct VehicleBehavior {
    actor_type: String,
    powertrain_enabled: bool,
    vehicle_fonction_enabled: bool,
    owner: Option<BaseObject>,
}

#[derive(Deserialize, Serialize)]
pub struct Vehicle {
    location: Vector3D,
    rotation: Rotator,
    velocity: Vector3D,
    acceleration: Vector3D,
    script_initialized: bool,
    hidden: bool,
    stasis: bool,
    health: f32,
    shield: f32,
    first_name: String,
    localized_last_name: i32,
    _unknown: Dummy<16>,
}

#[derive(Deserialize, Serialize, Default)]
struct WorldStreamingState {
    name: String,
    enabled: u8,
}

#[derive(Deserialize, Serialize)]
pub struct World {
    streaming_states: Vec<WorldStreamingState>,
    destination_area_map: String,
    destination: Vector3D,
    cinematics_seen: Vec<String>,
    scanned_clusters: Vec<i32>,
    scanned_systems: Vec<i32>,
    scanned_planets: Vec<i32>,
    journal_sort_method: u8,
    journal_showing_missions: bool,
    journal_last_selected_mission: i32,
    journal_last_selected_assignment: i32,
    codex_showing_primary: bool,
    codex_last_selected_primary: i32,
    codex_last_selected_secondary: i32,
    current_tip_id: i32,
    override_tip: i32,
    _browser_alerts: Dummy<8>, // [u8; 8]
    pending_loot: Option<BaseObject>,
}
