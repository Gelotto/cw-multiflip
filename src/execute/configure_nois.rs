use crate::{
  msg::ContractResult,
  state::{is_allowed, NOIS_PROXY_ADDR, USE_NOIS},
};
use cosmwasm_std::{attr, Addr, DepsMut, Env, MessageInfo, Response};

pub fn configure_nois(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  enabled: bool,
  maybe_proxy_addr: Option<Addr>,
) -> ContractResult<Response> {
  if !is_allowed(&deps.as_ref(), &info.sender, "use_nois")? {
    return Err(crate::error::ContractError::NotAuthorized {});
  }

  USE_NOIS.save(deps.storage, &enabled)?;

  if let Some(proxy_addr) = &maybe_proxy_addr {
    NOIS_PROXY_ADDR.save(deps.storage, proxy_addr)?;
  }

  Ok(Response::new().add_attributes(vec![
      attr("action", "use_nois"),
      attr("enabled", enabled.to_string()),
      attr(
        "proxy_address",
        maybe_proxy_addr
          .unwrap_or(NOIS_PROXY_ADDR.load(deps.storage)?)
          .to_string(),
      ),
    ]))
}
