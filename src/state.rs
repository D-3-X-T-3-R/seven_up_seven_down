use cosmwasm_std::Uint128;
use cw_storage_plus::Item;
use serde::{Deserialize, Serialize};

pub const STATE: Item<State> = Item::new("state");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct State {
    pub initial_balance: Uint128,
}
