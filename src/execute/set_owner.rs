use crate::{
  msg::ContractResult,
  state::{is_allowed, OWNER},
};
use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response};
use cw_lib::models::Owner;

pub fn set_owner(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  owner: Owner,
) -> ContractResult<Response> {
  if !is_allowed(&deps.as_ref(), &info.sender, "set_owner")? {
    return Err(crate::error::ContractError::NotAuthorized {});
  }

  OWNER.save(deps.storage, &owner)?;

  let owner_addr = match &owner {
    Owner::Address(addr) => addr,
    Owner::Acl(addr) => addr,
  };

  Ok(Response::new().add_attributes(vec![
    attr("action", "set_owner"),
    attr("owner", owner_addr.to_string()),
  ]))
}
