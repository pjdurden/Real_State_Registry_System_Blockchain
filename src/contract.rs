// use std::ops::Residual;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use cosmwasm_std::{
    to_binary, Addr, Binary, Coin, Deps, DepsMut, DistributionMsg, Env, MessageInfo, Response,
    StakingMsg, StdResult, Uint128, StdError,
};
use cw2::set_contract_version;
use cw_storage_plus::U64Key;

use crate::error::ContractError;
use crate::msg::{BenchmarkExecuteMsg, BenchmarkQueryMsg, InstantiateMsg};
use crate::state::{State, STATE, owner_of_registry_index};

// use terra_cosmwasm::TerraQuerier;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:gas-fees-benchmark";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        num_of_real_state:0,
        address_of_real_estate:vec![],
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: BenchmarkExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        BenchmarkExecuteMsg::ChangeOwnerofRealEstate{
            house_address,
            owner_name,
        }=>state_change_owner_of_real_estate(deps, _env,house_address,owner_name),
        BenchmarkExecuteMsg::PushRealEstateToBlockchain {
            house_address,
        } => state_push_realestate_to_blockchain(deps, _env, house_address),
        BenchmarkExecuteMsg::AddValidator {
            validator_addr,
            vault_denom,
        } => add_validator(deps, _env, info, validator_addr, vault_denom),
        BenchmarkExecuteMsg::StakingDelegate {
            validator_addr,
            denom,
            amount,
        } => state_staking_delegate(deps, _env, info, validator_addr, denom, amount),
        BenchmarkExecuteMsg::StakingUnDelegate {
            validator_addr,
            denom,
            amount,
        } => state_staking_undelegate(deps, _env, info, validator_addr, denom, amount),
        BenchmarkExecuteMsg::WithdrawRewards { validator_addr } => {
            state_withdraw_rewards(deps, _env, info, validator_addr)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: BenchmarkQueryMsg) -> StdResult<Binary> {
    match msg {
        BenchmarkQueryMsg::FindOwnerByHouseIndex {house_index} => to_binary(&query_owner_by_house_index(deps, _env,house_index)?),
        BenchmarkQueryMsg::FindOwnerByHouseName {house_name} => to_binary(&query_owner_by_house_name(deps, _env,house_name)?),
    }
}

fn state_push_realestate_to_blockchain(
    deps:DepsMut,
    _env: Env,
    house_address:String,
) -> Result<Response,ContractError> {
    let mut state=STATE.load(deps.storage)?;
    state.num_of_real_state+=1;
    state.address_of_real_estate.push((state.num_of_real_state,house_address));
    STATE.save(deps.storage,&state);
    Ok(Response::default())
}

fn state_change_owner_of_real_estate(
    deps:DepsMut,
    _env: Env,
    house_address:String,
    owner_name:String, 
) -> Result<Response,ContractError> {
    let mut house_index:u64 = 0; 
    let state=STATE.load(deps.storage)?;
    for house in state.address_of_real_estate{
        if house_address==house.1{
            house_index=house.0;
            break;
        }
    }
    if house_index==0{
        return Err(ContractError::RealEstateDoesNotExist{});
    }
    owner_of_registry_index.save(deps.storage,(house_index,house_address),&owner_name);

    Ok(Response::default())
}

fn query_owner_by_house_name(
    deps:Deps,
    _env: Env,
    house_name:String,
) -> StdResult<String>{
    let mut house_index:u64 = 0; 
    let state=STATE.load(deps.storage)?;
    for house in state.address_of_real_estate{
        if house_name==house.1{
            house_index=house.0;
            break;
        }
    }
    if house_index==0{
        return Err(StdError::GenericErr {
            msg: "House Not Found in Blockchain".to_string(),
        });
    }
    let house_result=owner_of_registry_index.load(deps.storage,
        ( house_index.clone(),house_name.clone() ) )?;

    Ok(house_result)
}

fn query_owner_by_house_index(
    deps:Deps,
    _env: Env,
    house_index:u64,
) -> StdResult<String>{
    let mut house_name:String = "Not Found".to_string(); 
    let mut flag:u64=0;
    let state=STATE.load(deps.storage)?;
    for house in state.address_of_real_estate{
        if house_index==house.0{
            house_name=house.1;
            flag=1;
            break;
        }
    }
    if flag==1 {
        return Err(StdError::GenericErr {
            msg: "House Not Found in Blockchain".to_string(),
        });
    }

    let house_result=owner_of_registry_index.load(deps.storage,
        (house_index.clone(),house_name.clone()) )?;

    Ok(house_result)

}


fn add_validator(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    validator_addr: Addr,
    vault_denom: String,
) -> Result<Response, ContractError> {
    // check if the validator exists in the blockchain
    if deps
        .querier
        .query_validator(validator_addr.clone())?
        .is_none()
    {
        return Err(ContractError::ValidatorDoesNotExist {});
    }

    let amount_to_stake_per_validator = Uint128::new(10);

    let funds = info.funds.first();
    if funds.is_none() {
        return Err(ContractError::InsufficientFunds {});
    }

    if funds.unwrap().amount.lt(&amount_to_stake_per_validator) {
        return Err(ContractError::InsufficientFunds {});
    }

    let msg = StakingMsg::Delegate {
        validator: validator_addr.to_string(),
        amount: Coin {
            denom: vault_denom.clone(),
            amount: amount_to_stake_per_validator,
        },
    };

    Ok(Response::new()
        .add_messages([msg])
        .add_attribute("method", "add_validator"))
}

fn state_staking_delegate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    validator_addr: Addr,
    vault_denom: String,
    amount_to_delegate: u64,
) -> Result<Response, ContractError> {
    let msg = StakingMsg::Delegate {
        validator: validator_addr.to_string(),
        amount: Coin {
            denom: vault_denom.clone(),
            amount: Uint128::new(amount_to_delegate.into()),
        },
    };
    Ok(Response::new().add_messages([msg]))
}

fn state_staking_undelegate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    validator_addr: Addr,
    vault_denom: String,
    amount_to_delegate: u64,
) -> Result<Response, ContractError> {
    let msg = StakingMsg::Undelegate {
        validator: validator_addr.to_string(),
        amount: Coin {
            denom: vault_denom.clone(),
            amount: Uint128::new(amount_to_delegate.into()),
        },
    };
    Ok(Response::new().add_messages([msg]))
}

fn  state_num_save(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    num_to_save_start:u64,
    num_to_save_end: u64,
) -> Result<Response, ContractError> {
    let mut state = STATE.load(deps.storage)?;
    for i in (num_to_save_start)..(num_to_save_end+1)
    {
        state.state_num = i;
    }
    STATE.save(deps.storage, &state);
    Ok(Response::default())
}

fn state_withdraw_rewards(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    validator_addr: Addr,
) -> Result<Response, ContractError> {
    // make sure the validator is added first
    let msg = DistributionMsg::WithdrawDelegatorReward {
        validator: validator_addr.to_string(),
    };
    Ok(Response::new().add_messages([msg]))
}
