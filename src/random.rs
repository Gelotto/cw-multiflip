use cosmwasm_std::{Storage, Uint128};
use nois::NoisCallback;

use crate::{error::ContractError, msg::ContractResult, state::NEXT_JOB_ID};

pub fn get_randomness(callback: &NoisCallback) -> ContractResult<[u8; 32]> {
  callback
    .randomness
    .to_array()
    .map_err(|_| ContractError::InvalidRandomness {})
}

pub fn get_next_job_id(storage: &mut dyn Storage) -> ContractResult<String> {
  Ok(
    (NEXT_JOB_ID.update(storage, |id| -> ContractResult<_> {
      Ok(id + Uint128::one())
    })?
      - Uint128::one())
    .to_string(),
  )
}
