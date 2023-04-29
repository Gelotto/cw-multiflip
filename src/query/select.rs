use crate::{
  models::FlippableCoinView,
  msg::{ContractResult, SelectResponse},
  state::{COINS, OWNER, USE_NOIS},
};
use cosmwasm_std::{Addr, Deps, Order};
use cw_repository::client::Repository;

pub fn select(
  deps: Deps,
  maybe_fields: Option<Vec<String>>,
  _wallet: Option<Addr>,
) -> ContractResult<SelectResponse> {
  let loader = Repository::loader(deps.storage, &maybe_fields);
  Ok(SelectResponse {
    owner: loader.get("owner", &OWNER)?,
    using_nois: loader.get("use_nois", &USE_NOIS)?,
    coins: loader.view("coins", || {
      Ok(Some(
        COINS
          .range(deps.storage, None, None, Order::Ascending)
          .map(|x| {
            let (i, c) = x.unwrap();
            FlippableCoinView {
              index: i,
              odds: c.odds,
              payout: c.payout,
              price: c.price,
            }
          })
          .collect(),
      ))
    })?,
  })
}
