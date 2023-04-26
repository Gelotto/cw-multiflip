use crate::{
  models::FlipCoinsJob,
  msg::ContractResult,
  random::get_randomness,
  state::{COINS, HOUSE_ADDR, JOBS},
};
use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response, Uint128, WasmMsg};
use cw_house_staking::client::House;
use nois::{ints_in_range, NoisCallback};

pub fn receive_randomness(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  callback: NoisCallback,
) -> ContractResult<Response> {
  let randomness = get_randomness(&callback)?;
  let job = JOBS.load(deps.storage, callback.job_id.clone())?;
  let observations: Vec<u16> = ints_in_range(randomness, job.n_flips as usize, 0, 1000);
  let mut resp = Response::new().add_attributes(vec![attr("action", "receive_randomness")]);
  if let Some(house_msg) = execute_flip_coins(deps, info, &job, observations)? {
    resp = resp.add_message(house_msg);
  }
  Ok(resp)
}

pub fn execute_flip_coins(
  deps: DepsMut,
  info: MessageInfo,
  job: &FlipCoinsJob,
  observations: Vec<u16>,
) -> ContractResult<Option<WasmMsg>> {
  let mut randomness_offset: usize = 0;
  let mut total_win = Uint128::zero();
  let mut total_loss = Uint128::zero();

  // flip each coin however many times the user flipped it
  // and add win and loss to running totals
  for flip in job.flips.iter() {
    let coin = COINS.load(deps.storage, flip.i_coin)?;
    let ints = &observations[randomness_offset..randomness_offset + flip.n_flips as usize];
    let n_wins = coin.flip(&ints);
    let win = coin.payout * Uint128::from(n_wins);
    let loss = coin.price * Uint128::from(flip.n_flips - n_wins);

    total_win += win;
    total_loss += loss;
    randomness_offset += flip.n_flips as usize;
  }

  // we're done with the job, so delete it
  JOBS.remove(deps.storage, job.job_id.clone());

  let house = House::new(&HOUSE_ADDR.load(deps.storage)?);

  // send payment to or from the house
  Ok(if total_win > total_loss {
    // payment to winner
    let amount = total_win - total_loss;
    Some(house.build_send_payment_msg(&info.sender, amount)?)
  } else if total_win < total_loss {
    // payment to house
    let amount = total_win - total_loss;
    Some(house.build_receive_payment_msg(amount, &info.funds)?)
  } else {
    None
  })
}
