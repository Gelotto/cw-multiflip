use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ContractError {
  #[error("{0}")]
  Std(#[from] StdError),

  #[error("NotAuthorized")]
  NotAuthorized {},

  #[error("InvalidNoisProxyAddress")]
  InvalidNoisProxyAddress {},

  #[error("InvalidRandomness")]
  InvalidRandomness {},

  #[error("ZeroFlipCount")]
  ZeroFlipCount {},

  #[error("InvalidCoinOdds")]
  InvalidCoinOdds {},

  #[error("InvalidCoinPayout")]
  InvalidCoinPayout {},

  #[error("InvalidCoinPrice")]
  InvalidCoinPrice {},
}
