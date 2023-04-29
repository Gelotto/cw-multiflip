use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Uint128, Uint64};

#[cw_serde]
pub struct FlippableCoinFace {
  pub name: String,
  pub image_url: String,
  pub n_wins: Option<Uint64>,
}

#[cw_serde]
pub struct FlippableCoin {
  pub price: Uint128,
  pub payout: Uint128,
  pub odds: u16,
  pub heads: FlippableCoinFace,
  pub tails: FlippableCoinFace,
}

#[cw_serde]
pub struct Flip {
  pub i_coin: u16,
  pub n_flips_heads: u16,
  pub n_flips_tails: u16,
  pub n_wins_heads: u16,
  pub n_wins_tails: u16,
}

#[cw_serde]
pub struct FlipCoinsJob {
  pub job_id: String,
  pub is_processed: bool,
  pub flips: Vec<Flip>,
  pub total_n_flips: u32,
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
    heads_observations: &[u16],
    tails_observations: &[u16],
  ) -> (u16, u16) {
    (
      heads_observations
        .iter()
        .map(|x| if *x < self.odds { 1 } else { 0 })
        .sum(),
      tails_observations
        .iter()
        .map(|x| if *x < self.odds { 1 } else { 0 })
        .sum(),
    )
  }
}
