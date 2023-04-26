use crate::{
  error::ContractError,
  models::{Flip, FlipCoinsJob},
  msg::{build_get_next_randomness_msg, ContractResult},
  random::get_next_job_id,
  state::{COINS, COINS_LEN, JOBS, USE_NOIS},
};
use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response, Storage, Uint128};
use cw_lib::random::{Pcg64, RngComponent};

use super::receive_randomness::execute_flip_coins;

pub fn flip_coins(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
  flips: &Vec<Flip>,
) -> ContractResult<Response> {
  let mut resp = Response::new().add_attributes(vec![attr("action", "flip_coins")]);
  let job = build_flip_coins_job(deps.storage, flips)?;

  if USE_NOIS.load(deps.storage)? {
    // Generate randomness asynchronously through IBC with Nois network.
    JOBS.save(deps.storage, job.job_id.to_string(), &job)?;
    resp = resp.add_message(build_get_next_randomness_msg(
      deps.storage,
      &job.job_id.clone(),
      &info.funds,
    )?);
  } else {
    // Generate randomness synchronously using PRNG.
    let mut rng = Pcg64::from_components(&vec![
      RngComponent::Int(env.block.height),
      RngComponent::Int(env.block.time.nanos()),
      RngComponent::Str(job.job_id.clone()),
      RngComponent::Int(if env.transaction.is_some() {
        env.transaction.unwrap().index as u64
      } else {
        0
      }),
    ]);
    // generate sequence random numbers equal in size to the aggregate number of
    // flips. Each observation is a value in the interval [0, 1000],
    // representing 0-100% in thousandths of a percent.
    let observations: Vec<u16> = {
      let mut v: Vec<u16> = Vec::with_capacity(job.n_flips as usize);
      for _ in 0..job.n_flips {
        v.push((rng.next_u64() % 1000) as u16)
      }
      v
    };
    // Add msg that tells the house to send or receive payment.  Note that it is
    // assumed at this point that the house has been granted a GLTO spending
    // allowance on behalf of the player via the CW20 contract's set_allowance
    // function...
    if let Some(house_msg) = execute_flip_coins(deps, info, &job, observations)? {
      resp = resp.add_message(house_msg);
    }
  }

  Ok(resp)
}

fn build_flip_coins_job(
  storage: &mut dyn Storage,
  flips: &Vec<Flip>,
) -> ContractResult<FlipCoinsJob> {
  let job_id = get_next_job_id(storage)?;
  let (n_flips, total_price) = compute_totals(storage, flips)?;
  Ok(FlipCoinsJob {
    job_id: job_id.clone(),
    flips: flips.clone(),
    total_price,
    n_flips,
  })
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
      return Err(ContractError::InvalidCoinOdds {});
    }
    let coin = COINS.load(storage, flip.i_coin)?;
    n_flips_total += flip.n_flips as u32;
    total_price = Uint128::from(flip.n_flips) * coin.price;
  }
  Ok((n_flips_total, total_price))
}
