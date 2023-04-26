use crate::{msg::ContractResult, state::is_allowed};
use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response};

pub fn use_nois(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  new_value: bool,
) -> ContractResult<Response> {
  if !is_allowed(&deps.as_ref(), &info.sender, "use_nois")? {
    return Err(crate::error::ContractError::NotAuthorized {});
  }
  Ok(Response::new().add_attributes(vec![
    attr("action", "use_nois"),
    attr("new_value", new_value.to_string()),
  ]))
}
