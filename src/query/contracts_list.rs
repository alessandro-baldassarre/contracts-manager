use archway_sdk::custom::query::ArchwayQuery;
use cosmwasm_std::{Deps, Order, StdResult};

use crate::state::{Contract, CONTRACTS_LIST};

pub fn query(deps: Deps<ArchwayQuery>) -> StdResult<Vec<(String, Contract<String>)>> {
    let contracts_res: StdResult<Vec<_>> = CONTRACTS_LIST
        .range(deps.storage, None, None, Order::Descending)
        .collect();
    let contracts_list = contracts_res?;
    let contracts_addr = contracts_list.clone().into_iter().map(|res| res.1.into());
    let contracts_label = contracts_list.into_iter().map(|res| res.0);
    let contracts = contracts_label
        .into_iter()
        .zip(contracts_addr.into_iter())
        .collect();
    Ok(contracts)
}
