pub use bet::{calculate_loss, calculate_prize, Bet};
use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response, StdResult};
pub use state::State;

mod bet;
mod contract;
mod state;
mod tests;
mod validations;

const CONTRACT_NAME: &str = "7up7down";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: contract::InstantiateMsg,
) -> StdResult<Response> {
    contract::instantiate(deps, env, info, msg)
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: contract::ExecuteMsg,
) -> StdResult<Response> {
    let mut counter: i32 = 3000;
    contract::execute(deps, env, info, msg, &mut counter)
}
