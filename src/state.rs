use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Uint128;

use cw_storage_plus::{Item, Map, U64Key};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub num_of_real_state: u64,
    pub address_of_real_estate: Vec<(u64,String)>,
}

pub const STATE: Item<State> = Item::new("state");

pub const owner_of_registry_index: Map<(u64, String), String> = Map::new("owner_of_registry");
// pub const owner_of_registry_address: Map<(u64, String), String> = Map::new("owner_of_registry");
