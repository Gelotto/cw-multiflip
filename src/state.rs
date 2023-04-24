use crate::models::FlipCoinsNoisJob;
use crate::msg::InstantiateMsg;
use crate::{error::ContractError, models::FlippableCoin};
use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, StdResult, Storage, Uint128};
use cw_lib::utils::validation::validate_addr;
use cw_storage_plus::{Item, Map};

pub const OWNER: Item<Addr> = Item::new("owner");
pub const COINS: Map<u16, FlippableCoin> = Map::new("coins");
pub const COINS_LEN: Item<u16> = Item::new("coins_len");
pub const JOBS: Map<String, FlipCoinsNoisJob> = Map::new("jobs");
pub const NEXT_JOB_ID: Item<Uint128> = Item::new("next_job_id");
pub const NOIS_PROXY_ADDR: Item<Addr> = Item::new("nois_proxy_addr");
pub const HOUSE_ADDR: Item<Addr> = Item::new("house_addr");

/// Initialize contract state data.
pub fn initialize(
  deps: DepsMut,
  _env: &Env,
  info: &MessageInfo,
  msg: &InstantiateMsg,
) -> Result<(), ContractError> {
  OWNER.save(deps.storage, &info.sender)?;
  COINS_LEN.save(deps.storage, &0)?;
  NEXT_JOB_ID.save(deps.storage, &Uint128::zero())?;
  NOIS_PROXY_ADDR.save(
    deps.storage,
    &validate_addr(
      deps.api,
      &msg.nois_proxy_addr,
      ContractError::InvalidNoisProxyAddress {},
    )?,
  )?;
  Ok(())
}

pub fn is_owner(
  storage: &dyn Storage,
  addr: &Addr,
) -> StdResult<bool> {
  return Ok(OWNER.load(storage)? == *addr);
}
