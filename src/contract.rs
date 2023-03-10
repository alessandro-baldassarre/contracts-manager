use archway_sdk::custom::query::ArchwayQuery;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdError, StdResult,
};
use cw2::set_contract_version;
use cw_controllers::Admin;

use crate::error::ContractError;
use crate::execute::instantiate_contract::INSTANTIATE_REPLY_ID;
use crate::execute::{change_owner, instantiate_contract, set_contract_metadata};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query::{contract_metadata, contracts_list, rewards_record};
use crate::reply::instantiate;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:contracts-manager";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const ADMIN: Admin = Admin::new("admin");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<ArchwayQuery>,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // Set the contract version for future migration
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // Calculate the address to set for owner (if not set in the message takes the sender address
    // by default)
    let owner_addr = msg
        .owner
        .map(|addr| deps.api.addr_validate(&addr))
        .transpose()?
        .unwrap_or(info.sender);
    ADMIN.set(deps, Some(owner_addr))?;

    Ok(Response::new().add_attribute("action", "instantiated_contract_manager"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<ArchwayQuery>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::InstantiateContract {
            code_id,
            instantiate_msg,
            label,
        } => instantiate_contract::execute(deps, env, info, code_id, instantiate_msg, label),
        ExecuteMsg::ChangeOwner { new_owner } => change_owner::execute(deps, info, new_owner),
        ExecuteMsg::SetContractMetadata {
            contract_address,
            rewards_address,
        } => set_contract_metadata::execute(deps, env, info, contract_address, rewards_address),

        _ => unimplemented!(),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<ArchwayQuery>, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::ContractMetadata { contract_address } => {
            to_binary(&contract_metadata::query(deps, contract_address)?)
        }
        QueryMsg::RewardsRecord { pagination } => {
            to_binary(&rewards_record::query(deps, env, pagination)?)
        }
        QueryMsg::ContractsList {} => to_binary(&contracts_list::query(deps)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        INSTANTIATE_REPLY_ID => instantiate::reply(deps, msg),
        id => Err(ContractError::Std(StdError::GenericErr {
            msg: format!("Unknown reply id: {}", id),
        })),
    }
}
