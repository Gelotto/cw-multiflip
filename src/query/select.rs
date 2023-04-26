use crate::{
  models::FlippableCoinView,
  msg::SelectResponse,
  state::{COINS, OWNER},
};
use cosmwasm_std::{Addr, Deps, Order, StdResult};
use cw_repository::client::Repository;

pub fn select(
  deps: Deps,
  maybe_fields: Option<Vec<String>>,
  _wallet: Option<Addr>,
) -> StdResult<SelectResponse> {
  let loader = Repository::loader(deps.storage, &maybe_fields);
  Ok(SelectResponse {
    owner: loader.get("owner", &OWNER)?,
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
