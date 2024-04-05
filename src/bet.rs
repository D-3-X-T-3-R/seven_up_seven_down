use cosmwasm_std::Uint128;
use cw_storage_plus::Map;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct Bet {
    pub prediction: u8,
    pub amount: Uint128,
}

pub const BETS: Map<&[u8], Bet> = Map::new("bets");

pub fn calculate_prize(prediction: u8, amount: Uint128) -> Uint128 {
    match prediction {
        3 => amount * Uint128::from(5u128), // Five times the amount for exact match
        _ => amount * Uint128::from(2u128), // Double the amount for other predictions
    }
}

pub fn calculate_loss(prediction: u8, amount: Uint128) -> Uint128 {
    match prediction {
        3 => amount * Uint128::from(2u128), // Double the bet amount for exact match
        _ => amount,                        // Loss is same as the bet amount for other predictions
    }
}
