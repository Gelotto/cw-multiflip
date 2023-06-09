use crate::error::ContractError;
use crate::execute;
use crate::msg::{ContractResult, ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::query;
use crate::state;
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;

const CONTRACT_NAME: &str = "crates.io:cw-multiflip";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn instantiate(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
  msg: InstantiateMsg,
) -> Result<Response, ContractError> {
  set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
  state::initialize(deps, &env, &info, &msg)?;
  Ok(Response::new().add_attribute("action", "instantiate"))
}

#[entry_point]
pub fn execute(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
  msg: ExecuteMsg,
) -> Result<Response, ContractError> {
  match msg {
    ExecuteMsg::SetOwner { owner } => execute::set_owner(deps, env, info, owner),
    ExecuteMsg::ConfigureNois {
      enabled,
      proxy_address,
    } => execute::configure_nois(deps, env, info, enabled, proxy_address),
    ExecuteMsg::FlipCoins { flips } => execute::flip_coins(deps, env, info, &flips),
    ExecuteMsg::ReceiveRandomness { callback } => {
      execute::receive_randomness(deps, env, info, callback)
    },
  }
}

#[entry_point]
pub fn query(
  deps: Deps,
  _env: Env,
  msg: QueryMsg,
) -> ContractResult<Binary> {
  let result = match msg {
    QueryMsg::Select { fields, wallet } => to_binary(&query::select(deps, fields, wallet)?),
    QueryMsg::Job { job_id } => to_binary(&query::get_job(deps, job_id)?),
  }?;
  Ok(result)
}

#[entry_point]
pub fn migrate(
  _deps: DepsMut,
  _env: Env,
  _msg: MigrateMsg,
) -> Result<Response, ContractError> {
  Ok(Response::default())
}
