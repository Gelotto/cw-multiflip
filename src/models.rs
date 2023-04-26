use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;

#[cw_serde]
pub struct FlippableCoin {
  pub price: Uint128,
  pub payout: Uint128,
  pub odds: u16,
}

#[cw_serde]
pub struct Flip {
  pub i_coin: u16,
  pub n_flips: u16,
}

#[cw_serde]
pub struct FlipCoinsJob {
  pub job_id: String,
  pub flips: Vec<Flip>,
  pub n_flips: u32,
  pub total_price: Uint128,
}

#[cw_serde]
pub struct FlippableCoinView {
  pub index: u16,
  pub price: Uint128,
  pub payout: Uint128,
  pub odds: u16,
}

impl FlippableCoin {
  /// flip the coin N times, returning the number of flips won by the player.
  pub fn flip(
    &self,
    observations: &[u16],
  ) -> u16 {
    observations
      .iter()
      .map(|x| if *x < self.odds { 1 } else { 0 })
      .sum()
  }
}
