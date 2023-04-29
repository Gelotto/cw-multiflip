use crate::{
  error::ContractError,
  models::FlipCoinsJob,
  msg::ContractResult,
  random::get_randomness,
  state::{COINS, HOUSE_ADDR, JOBS},
};
use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response, Uint128, Uint64, WasmMsg};
use cw_house_staking::client::House;
use nois::{ints_in_range, NoisCallback};

pub fn receive_randomness(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  callback: NoisCallback,
) -> ContractResult<Response> {
  let mut job = JOBS.load(deps.storage, callback.job_id.clone())?;

  if job.is_processed {
    return Err(ContractError::JobAlreadyProcessed);
  }

  let randomness = get_randomness(&callback)?;
  let observations: Vec<u16> = ints_in_range(randomness, job.total_n_flips as usize, 0, 999);

  let mut resp = Response::new().add_attributes(vec![attr("action", "receive_randomness")]);

  if let Some(house_msg) = execute_flip_coins(deps, info, &mut job, observations)? {
    resp = resp.add_message(house_msg);
  }
  Ok(resp)
}

pub fn execute_flip_coins(
  deps: DepsMut,
  info: MessageInfo,
  job: &mut FlipCoinsJob,
  observations: Vec<u16>,
) -> ContractResult<Option<WasmMsg>> {
  let mut randomness_offset: usize = 0;
  let mut total_win = Uint128::zero();
  let mut total_loss = Uint128::zero();

  job.is_processed = true;

  // flip each coin however many times the user flipped it
  // and add win and loss to running totals
  for flip in job.flips.iter_mut() {
    let n_flips = flip.n_flips_heads + flip.n_flips_tails;
    COINS.update(
      deps.storage,
      flip.i_coin,
      |maybe_coin| -> ContractResult<_> {
        if let Some(mut coin) = maybe_coin {
          let heads_observations =
            &observations[randomness_offset..randomness_offset + flip.n_flips_heads as usize];

          randomness_offset += flip.n_flips_heads as usize;

          let tails_observations =
            &observations[randomness_offset..randomness_offset + flip.n_flips_tails as usize];

          randomness_offset += flip.n_flips_tails as usize;

          let (n_wins_heads, n_wins_tails) = coin.flip(&heads_observations, &tails_observations);
          let n_wins = n_wins_heads + n_wins_tails;

          let win = coin.payout * Uint128::from(n_wins);
          let loss = coin.price * Uint128::from(n_flips - n_wins);

          total_win += win;
          total_loss += loss;

          flip.n_wins_heads = flip.n_wins_heads;
          flip.n_wins_tails = flip.n_wins_tails;

          coin.heads.n_wins =
            Some(coin.heads.n_wins.unwrap_or_default() + Uint64::from(n_wins_heads));

          coin.tails.n_wins =
            Some(coin.tails.n_wins.unwrap_or_default() + Uint64::from(n_wins_tails));

          Ok(coin)
        } else {
          Err(ContractError::CoinIndexOutOfBounds)
        }
      },
    )?;
  }

  // we're done with the job, so delete it
  JOBS.save(deps.storage, job.job_id.clone(), &job)?;

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
