use crate::{msg::SelectResponse, state::OWNER};
use cosmwasm_std::{Addr, Deps, StdResult};

pub fn select(
  deps: Deps,
  fields: Option<Vec<String>>,
  _wallet: Option<Addr>,
) -> StdResult<SelectResponse> {
  if let Some(fields) = fields {
    Ok(SelectResponse {
      owner: if fields.contains(&"owner".to_owned()) {
        OWNER.may_load(deps.storage)?
      } else {
        None
      },
    })
  } else {
    Ok(SelectResponse {
      owner: OWNER.may_load(deps.storage)?,
    })
  }
}
