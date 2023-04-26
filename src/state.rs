use crate::models::FlipCoinsJob;
use crate::msg::{ContractResult, InstantiateMsg};
use crate::{error::ContractError, models::FlippableCoin};
use cosmwasm_std::{Addr, Deps, DepsMut, Env, MessageInfo, Uint128};
use cw_acl::client::Acl;
use cw_lib::models::Owner;
use cw_lib::utils::validation::validate_addr;
use cw_storage_plus::{Item, Map};

pub const PUBLIC_NOIS_PROXY_ADDR: &str =
  "juno1qr84ktm57q5t02u04ddk5r8s79axdzglad6tfdd9g2xgt4hkh6jsgeq9x2";

pub const OWNER: Item<Owner> = Item::new("owner");
pub const COINS: Map<u16, FlippableCoin> = Map::new("coins");
pub const COINS_LEN: Item<u16> = Item::new("coins_len");
pub const JOBS: Map<String, FlipCoinsJob> = Map::new("jobs");
pub const NEXT_JOB_ID: Item<Uint128> = Item::new("next_job_id");
pub const USE_NOIS: Item<bool> = Item::new("use_nois");
pub const NOIS_PROXY_ADDR: Item<Addr> = Item::new("nois_proxy_addr");
pub const HOUSE_ADDR: Item<Addr> = Item::new("house_addr");

pub fn initialize(
  deps: DepsMut,
  _env: &Env,
  _info: &MessageInfo,
  msg: &InstantiateMsg,
) -> Result<(), ContractError> {
  OWNER.save(deps.storage, &msg.owner)?;
  COINS_LEN.save(deps.storage, &0)?;
  NEXT_JOB_ID.save(deps.storage, &Uint128::zero())?;
  USE_NOIS.save(deps.storage, &msg.use_nois.unwrap_or(false))?;
  HOUSE_ADDR.save(
    deps.storage,
    &validate_addr(
      deps.api,
      &msg.house_addr,
      ContractError::InvalidNoisProxyAddress {},
    )?,
  )?;
  NOIS_PROXY_ADDR.save(
    deps.storage,
    &validate_addr(
      deps.api,
      &msg
        .nois_proxy_addr
        .clone()
        .unwrap_or(Addr::unchecked(PUBLIC_NOIS_PROXY_ADDR)),
      ContractError::InvalidNoisProxyAddress {},
    )?,
  )?;

  for (i, coin) in msg.coins.iter().enumerate() {
    validate_coin(coin)?;
    COINS.save(deps.storage, i as u16, &coin)?;
  }

  Ok(())
}

pub fn validate_coin(coin: &FlippableCoin) -> ContractResult<()> {
  if coin.odds < 1 || coin.odds > 1000 {
    return Err(ContractError::InvalidCoinOdds {});
  }
  if coin.payout.is_zero() {
    return Err(ContractError::InvalidCoinPayout {});
  }
  if coin.price.is_zero() {
    return Err(ContractError::InvalidCoinPrice {});
  }
  Ok(())
}

pub fn is_allowed(
  deps: &Deps,
  principal: &Addr,
  action: &str,
) -> ContractResult<bool> {
  Ok(match OWNER.load(deps.storage)? {
    Owner::Address(addr) => *principal == addr,
    Owner::Acl(acl_addr) => {
      let acl = Acl::new(&acl_addr);
      acl.is_allowed(&deps.querier, principal, action)?
    },
  })
}
