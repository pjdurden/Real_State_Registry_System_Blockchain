use std::ops::Residual;

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



// fn state_exec_nums_load(
//     deps: DepsMut ,
//     _env: Env ,
//     _info: MessageInfo ,
//  ) -> Result<Response,ContractError> {
//     let state = STATE.load(deps.storage)?;
//     Ok(Response::default())

// }

// fn state_exec_vectors_load(
//     deps: DepsMut ,
//     _env: Env ,
//     _info: MessageInfo ,
//  ) -> Result<Response,ContractError> {
//     let state = STATE.load(deps.storage)?;
//     let vector_to_find=state.state_vector;
//     Ok(Response::default())

// }

// fn state_exec_vectors_load_sorted(
//     deps: DepsMut ,
//     _env: Env ,
//     _info: MessageInfo ,
//  ) -> Result<Response,ContractError> {
//     let state = STATE.load(deps.storage)?;
//     let mut vector_to_find=state.state_vector;
//     vector_to_find.sort();
//     Ok(Response::default())

// }

// fn state_exec_vectors_save_sorted(
//     deps: DepsMut ,
//     _env: Env ,
//     _info: MessageInfo ,
//  ) -> Result<Response,ContractError> {
//     let mut state = STATE.load(deps.storage)?;
//     let mut vector_to_find=state.state_vector;
//     vector_to_find.sort();
//     state.state_vector=vector_to_find;
//     STATE.save(deps.storage,&state);
//     Ok(Response::default())

// }

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

// fn state_vector_save(
//     deps: DepsMut,
//     _env: Env,
//     _info: MessageInfo,
//     num_to_save_start:u64,
//     num_to_save_end: u64,
// ) -> Result<Response, ContractError> {
//     let mut state = STATE.load(deps.storage)?;
//     for i in (num_to_save_start)..(num_to_save_end+1)
//     {
//         state.state_vector.push(i);
//     }
//     STATE.save(deps.storage, &state);
//     Ok(Response::default())
// }

// fn state_num_update(
//     deps: DepsMut,
//     _env: Env,
//     _info: MessageInfo,
//     num_to_update_start: u64,
//     num_to_update_end:u64,
// ) -> Result<Response, ContractError> {
//     for x in num_to_update_start..(num_to_update_end+1)
//     {
//         STATE.update(deps.storage, |mut s| -> StdResult<_> {
//             s.state_num = x;
//             Ok(s)
//         })?;
//     }
//     Ok(Response::default())
// }

// fn state_vector_update(
//     deps: DepsMut,
//     _env: Env,
//     _info: MessageInfo,
//     num_to_update_start: u64,
//     num_to_update_end:u64,
// ) -> Result<Response, ContractError> {
//     for x in num_to_update_start..(num_to_update_end+1)
//     {
//         STATE.update(deps.storage, |mut s| -> StdResult<_> {
//             s.state_vector.push(x);
//             Ok(s)
//         })?;
//     }
//     Ok(Response::default())
// }

// fn state_composite_key_update(
//     deps: DepsMut,
//     _env: Env,
//     _info: MessageInfo,
//     first_key_start:u64,
//     second_key_start:u64,
//     value_start:u64,
//     first_key_end:u64,
//     second_key_end:u64,
//     value_end:u64,
// ) -> Result<Response, ContractError> {
//     // make sure to save the value at first_key first before updating
//     // checking by loading the map will add the gas fees of loading too

//     let mut first_key=first_key_start;
//     let mut second_key=second_key_start;
//     for x in value_start..(value_end+1)
//     {
//         MAP_COMPOSITE_KEY.update(
//             deps.storage,
//             (U64Key::new(first_key), U64Key::new(second_key)),
//             |_v| -> StdResult<Uint128> { Ok(x.into()) },
//         )?;
//         first_key=first_key+1;
//         second_key=second_key+1;
//     }

    
//     Ok(Response::default())
// }

// fn state_composite_key_save(
//     deps: DepsMut,
//     _env: Env,
//     _info: MessageInfo,
//     first_key_start:u64,
//     second_key_start:u64,
//     value_start:u64,
//     first_key_end:u64,
//     second_key_end:u64,
//     value_end:u64,
// ) -> Result<Response, ContractError> {
//     let mut first_key=first_key_start;
//     let mut second_key=second_key_start;
//     for x in value_start..(value_end+1)
//     {
//         MAP_COMPOSITE_KEY.save(
//             deps.storage,
//             (U64Key::new(first_key), U64Key::new(second_key)),
//             &(x.into()),
//         )?;
//         first_key=first_key+1;
//         second_key=second_key+1;
//     }
    

//     Ok(Response::default())
// }

// fn state_vector_value_save(
//     deps: DepsMut,
//     _env: Env,
//     _info: MessageInfo,
//     first_key_start:u64,
//     second_key_start:u64,
//     value_start:u64,
//     first_key_end:u64,
//     second_key_end:u64,
//     value_end:u64,
// ) -> Result<Response, ContractError> {
    
    
//     let mut first_key=first_key_start;
//     let mut second_key=second_key_start;

//     for x in value_start..(value_end+1)
//     {
//         let mut vec_to_save = MAP_VECTOR_VALUE.load(deps.storage, U64Key::new(first_key))?;
//         vec_to_save.push((second_key, x.into()));
//         MAP_VECTOR_VALUE.save(deps.storage, U64Key::new(first_key), &vec_to_save)?;
//         first_key=first_key+1;
//         second_key=second_key+1;
//     }
    
//     Ok(Response::default())
// }

// fn state_vector_value_update(
//     deps: DepsMut,
//     _env: Env,
//     _info: MessageInfo,
//     first_key_start:u64,
//     second_key_start:u64,
//     value_start:u64,
//     first_key_end:u64,
//     second_key_end:u64,
//     value_end:u64,
// ) -> Result<Response, ContractError> {
//     // save the value in map before updating

//     let mut first_key=first_key_start;
//     let mut second_key=second_key_start;

//     for x in value_start..(value_end+1)
//     {
//         let mut vec_to_update = MAP_VECTOR_VALUE.load(deps.storage, U64Key::new(first_key))?;
//         vec_to_update.push((second_key, x.into()));
//         MAP_VECTOR_VALUE.update(
//             deps.storage,
//             U64Key::new(first_key),
//             |_v| -> StdResult<Vec<(u64, Uint128)>> { Ok(vec_to_update) },
//         )?;
//         first_key=first_key+1;
//         second_key=second_key+1;
//     }

   
//     Ok(Response::default())
// }

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

// fn query_state_num_load(deps: Deps, _env: Env) -> StdResult<u64> {
//     let state = STATE.load(deps.storage)?;
//     Ok(state.state_num)
// }

// fn query_state_vector_load(deps: Deps, _env: Env) -> StdResult<Vec<u64>> {
//     let state = STATE.load(deps.storage)?;
//     Ok(state.state_vector)
// }

// fn query_state_vector_load_sorted(deps: Deps, _env: Env) -> StdResult<Vec<u64>> {
//     let state = STATE.load(deps.storage)?;
//     let mut vec_to_sort = state.state_vector;
//     vec_to_sort.sort();
//     Ok(vec_to_sort)
// }

// fn query_map_composite_key_load(
//     deps: Deps,
//     _env: Env,
//     first_key: u64,
//     second_key: u64,
// ) -> StdResult<Uint128> {
//     let value_to_find = MAP_COMPOSITE_KEY.load(
//         deps.storage,
//         (U64Key::new(first_key), U64Key::new(second_key)),
//     )?;
//     Ok(value_to_find)
// }

// fn query_map_vector_value_load(
//     deps: Deps,
//     _env: Env,
//     num_to_find: u64,
// ) -> StdResult<Vec<(u64, Uint128)>> {
//     let value_to_find = MAP_VECTOR_VALUE.load(deps.storage, U64Key::new(num_to_find))?;
//     Ok(value_to_find)
// }
