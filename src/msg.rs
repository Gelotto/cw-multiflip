use cosmwasm_schema::cw_serde;
use cosmwasm_std::{to_binary, Addr, Coin, Storage, WasmMsg};
use nois::{NoisCallback, ProxyExecuteMsg};

use crate::{
  error::ContractError,
  models::{Flip, FlippableCoin},
  state::NOIS_PROXY_ADDR,
};

pub type ContractResult<T> = Result<T, ContractError>;

#[cw_serde]
pub struct InstantiateMsg {
  pub nois_proxy_addr: Addr,
  pub coins: Vec<FlippableCoin>,
  pub columns: u16,
}

#[cw_serde]
pub enum ExecuteMsg {
  FlipCoins { flips: Vec<Flip> },
  ReceiveRandomness { callback: NoisCallback },
}

#[cw_serde]
pub enum QueryMsg {
  Select {
    fields: Option<Vec<String>>,
    wallet: Option<Addr>,
  },
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct SelectResponse {
  pub owner: Option<Addr>,
}

pub fn build_get_next_randomness_msg(
  storage: &dyn Storage,
  job_id: &String,
  funds: &Vec<Coin>,
) -> ContractResult<WasmMsg> {
  Ok(WasmMsg::Execute {
    contract_addr: NOIS_PROXY_ADDR.load(storage)?.into(),
    msg: to_binary(&ProxyExecuteMsg::GetNextRandomness {
      job_id: job_id.clone(),
    })?,
    funds: funds.clone(),
  })
}
