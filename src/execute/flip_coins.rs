use crate::{
  error::ContractError,
  models::{Flip, FlipCoinsNoisJob},
  msg::{build_get_next_randomness_msg, ContractResult},
  random::get_next_job_id,
  state::{COINS, COINS_LEN, JOBS},
};
use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response, Storage, Uint128};

pub fn flip_coins(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  flips: &Vec<Flip>,
) -> ContractResult<Response> {
  let job = create_flip_coins_nois_job(deps.storage, flips)?;
  // TODO: build token transfer msg to take payment
  Ok(
    Response::new()
      .add_attributes(vec![attr("action", "flip_coins")])
      .add_message(build_get_next_randomness_msg(
        deps.storage,
        &job.job_id.clone(),
        &info.funds,
      )?),
  )
}

fn create_flip_coins_nois_job(
  storage: &mut dyn Storage,
  flips: &Vec<Flip>,
) -> ContractResult<FlipCoinsNoisJob> {
  let job_id = get_next_job_id(storage)?;
  let (n_flips, total_price) = compute_totals(storage, flips)?;
  let job = FlipCoinsNoisJob {
    job_id: job_id.clone(),
    flips: flips.clone(),
    total_price,
    n_flips,
  };
  JOBS.save(storage, job_id.to_string(), &job)?;
  Ok(job)
}

fn compute_totals(
  storage: &dyn Storage,
  flips: &Vec<Flip>,
) -> ContractResult<(u32, Uint128)> {
  let n_coins = COINS_LEN.load(storage)?;
  let mut n_flips_total: u32 = 0;
  let mut total_price = Uint128::zero();
  for flip in flips.iter() {
    if flip.n_flips == 0 {
      return Err(ContractError::ZeroFlipCount {});
    }
    if flip.i_coin >= n_coins {
      return Err(ContractError::CoinIndexOutOfBounds {});
    }
    let coin = COINS.load(storage, flip.i_coin)?;
    n_flips_total += flip.n_flips as u32;
    total_price = Uint128::from(flip.n_flips) * coin.price;
  }
  Ok((n_flips_total, total_price))
}
