use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum BenchmarkExecuteMsg {

    PushRealEstateToBlockchain {house_address:String},
    ChangeOwnerofRealEstate{house_address:String,owner_name:String},
    AddValidator {
        validator_addr: Addr,
        vault_denom: String,
    },
    StakingDelegate {
        validator_addr: Addr,
        denom: String,
        amount: u64,
    },
    StakingUnDelegate {
        validator_addr: Addr,
        denom: String,
        amount: u64,
    },
    WithdrawRewards {
        validator_addr: Addr,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum BenchmarkQueryMsg {
    FindOwnerByHouseIndex {house_index:u64},
    FindOwnerByHouseName {house_name:String},
}
