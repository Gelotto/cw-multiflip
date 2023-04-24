use crate::{msg::SelectResponse, state::OWNER};
use cosmwasm_std::{Addr, Deps, StdResult};

pub fn select(
  deps: Deps,
  _fields: Option<Vec<String>>,
  _wallet: Option<Addr>,
) -> StdResult<SelectResponse> {
  Ok(SelectResponse {
    owner: OWNER.may_load(deps.storage)?,
  })
}
