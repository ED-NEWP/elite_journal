use serde::Deserialize;
use crate::Coordinate;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct System {
    pub star_system: String,
    pub system_address: u64,
    pub star_pos: Coordinate,
    pub system_allegiance: String,
    pub system_economy: String,
    pub system_second_economy: String,
    pub system_government: String,
    pub system_security: String,
    pub population: u64,
    pub system_faction: Option<Faction>,
    #[serde(default)]
    pub factions: Vec<FactionInfo>,
    #[serde(default)]
    pub conflicts: Vec<FactionConflict>,
    pub powers: Option<Vec<String>>,
    pub powerplay_state: Option<PowerplayState>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct FsdJump {
    #[serde(flatten)]
    pub system: System,
    // EDDN optional only?
    pub jump_dist: Option<f64>,
    // EDDN optional only?
    pub fuel_used: Option<f64>,
    pub fuel_level: Option<f64>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Location {
    #[serde(flatten)]
    pub system: System,
    pub body: String,
    #[serde(rename = "BodyID")]
    pub body_id: u64,
    pub body_type: String,  // TODO: Enum?
    pub dist_from_star_ls: Option<f64>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub docked: bool,
    pub station_name: Option<String>,
    pub station_type: Option<String>,  // TODO: Enum?
    #[serde(rename = "MarketID")]
    pub market_id: Option<u64>,

    pub station_faction: Option<Faction>,
    pub station_government: Option<String>,  // TODO: Enum?
    pub station_allegiance: Option<String>,  // TODO: Enum?
    pub station_services: Option<Vec<String>>,  // TODO: Enums??
    pub station_economies: Option<Vec<Economy>>,  // ???? (Array of (Name,Proportion) pairs )

    pub wanted: Option<bool>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Faction {
    pub name: String,
    #[serde(rename = "FactionState")]
    pub state: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct FactionInfo {
    pub name: String,
    #[serde(rename = "FactionState")]
    pub state: String,
    pub government: String,
    pub influence: f64,
    pub allegiance: String,
    pub happiness: String,
    #[serde(default)]
    pub pending_states: Vec<FactionStateTrend>,
    #[serde(default)]
    pub active_states: Vec<FactionStateTrend>,
    #[serde(default)]
    pub recovering_states: Vec<FactionStateTrend>,
    // EDDN optional only?
    #[serde(rename = "MyReputation")]
    pub reputation: Option<f64>,
    #[serde(default)]
    pub squadron_faction: bool,
    #[serde(default)]
    pub home_system: bool,
    #[serde(default)]
    pub happiest_system: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FactionStateTrend {
    pub state: String,
    pub trend: Option<u8>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct FactionConflict {
    pub war_type: String,
    pub status: String,
    pub faction_1: FactionConflictProgress,
    pub faction_2: FactionConflictProgress,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct FactionConflictProgress {
    pub name: String,
    pub stake: String,
    pub won_days: u8,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Economy {
    pub name: String,
    pub proportion: f64,
}

#[derive(Deserialize, Debug)]
pub enum PowerplayState {
    InPrepareRadius,
    Prepared,
    Exploited,
    Contested,
    Controlled,
    Turmoil,
    HomeSystem,
}
