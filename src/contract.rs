use crate::{
    bet::BETS, calculate_loss, calculate_prize, state::STATE, validations::validate_bet_amount,
    Bet, State, CONTRACT_NAME, CONTRACT_VERSION,
};
use cosmwasm_std::{Addr, BankMsg, Coin, DepsMut, Env, MessageInfo, Response, StdResult, Uint128};
use cw2::set_contract_version;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InstantiateMsg {
    pub initial_balance: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    PlaceBet { prediction: u8, amount: Uint128 },
}

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let state = State {
        initial_balance: msg.initial_balance,
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;
    Ok(Response::new().add_attribute("method", "instantiate"))
}

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
    mut counter: &mut i32,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::PlaceBet { prediction, amount } => {
            place_bet(deps, env, info, prediction, amount, &mut counter)
        }
    }
}

fn place_bet(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    prediction: u8,
    amount: Uint128,
    mut counter: &mut i32,
) -> StdResult<Response> {
    validate_bet_amount(amount, prediction, &info)?;

    let sender_addr = info.sender.clone();
    let contract_addr = env.contract.address.clone();
    let bet_id = generate_bet_id(&mut counter);

    BETS.save(deps.storage, bet_id.as_bytes(), &Bet { prediction, amount })?;

    let roll = simulate_dice_roll(&mut counter);
    let win = is_winner(prediction, roll);

    let (winner_addr, total_amount) = if win {
        (sender_addr, calculate_prize(prediction, amount))
    } else {
        (contract_addr, calculate_loss(prediction, amount))
    };

    // let coin = vec![Coin {
    //     denom: "token".to_string(),
    //     amount: total_amount,
    // }];

    // let response_msg = BankMsg::Send {
    //     to_address: winner_addr.into_string(),
    //     amount: coin,
    // };
    let response_msg = create_transfer_msg(winner_addr, total_amount);

    Ok(Response::new()
        .add_message(response_msg)
        .add_attribute("method", "place_bet")
        .add_attribute("roll", roll.to_string())
        .add_attribute("win", win.to_string())
        .add_attribute("bet_id", bet_id))
}

pub fn simulate_dice_roll(mut counter: &mut i32) -> u8 {
    // let mut rng = rand::thread_rng();
    // rng.gen_range(1..=6) + rng.gen_range(1..=6)

    let num = *counter % 7_i32;
    return num as u8;
}

pub fn is_winner(prediction: u8, roll: u8) -> bool {
    match prediction {
        1 => roll > 7,
        2 => roll < 7,
        3 => roll == 7,
        _ => false,
    }
}

pub fn generate_bet_id(mut counter: &mut i32) -> String {
    // let base = format!("{}_{}", env.block.time.seconds(), info.sender);

    // let rand_suffix: String = rand::thread_rng()
    //     .sample_iter(&Alphanumeric)
    //     .take(16)
    //     .map(char::from)
    //     .collect();

    // format!("{}_{}", base, rand_suffix)
    (*counter + 1_i32).to_string()
}

pub fn create_transfer_msg(winner_addr: Addr, amount: Uint128) -> BankMsg {
    BankMsg::Send {
        to_address: winner_addr.to_string(),
        amount: vec![Coin {
            denom: "ucmdx".to_string(),
            amount,
        }],
    }
}
