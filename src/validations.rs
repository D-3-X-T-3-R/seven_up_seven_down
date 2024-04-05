use cosmwasm_std::{MessageInfo, StdError, StdResult, Uint128};

pub fn validate_bet_amount(amount: Uint128, prediction: u8, info: &MessageInfo) -> StdResult<()> {
    if amount.is_zero() {
        return Err(StdError::generic_err(
            "Bet amount must be greater than zero",
        ));
    }

    let sender_balance = info
        .funds[0].amount;
        // .iter()
        // .find(|coin| coin.denom == "token")
        // .map(|coin| coin.amount)
        // .unwrap_or_default();

    if prediction == 3 {
        let required_balance = amount * Uint128::from(2u128);
        if sender_balance < required_balance {
            return Err(StdError::generic_err(
                "Insufficient balance to place the bet",
            ));
        }
    } else {
        if sender_balance < amount {
            return Err(StdError::generic_err(
                "Insufficient balance to place the bet",
            ));
        }
    }

    Ok(())
}
