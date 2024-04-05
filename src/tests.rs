#[cfg(test)]
mod integration_tests {
    mod contract_tests {
        use crate::bet::BETS;
        use crate::contract::{
            create_transfer_msg, generate_bet_id, is_winner, simulate_dice_roll, ExecuteMsg,
            InstantiateMsg,
        };
        use crate::{execute, instantiate, Bet};

        use super::*;
        use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
        use cosmwasm_std::{attr, Addr, BankMsg, Coin, MessageInfo, Response, StdResult, Uint128};

        // Helper function to provide coins
        fn coins(amount: u128, denom: &str) -> Vec<Coin> {
            vec![Coin {
                denom: denom.to_string(),
                amount: Uint128::new(amount),
            }]
        }

        #[test]
        fn dice_roll_in_range() {
            for _ in 0..100 {
                let roll = simulate_dice_roll(&mut 3000);
                assert!(roll >= 2 && roll <= 12, "Dice roll out of range: {}", roll);
            }
        }

        #[test]
        fn winner_logic() {
            assert!(is_winner(1, 8));
            assert!(!is_winner(1, 6));
            assert!(is_winner(2, 6));
            assert!(!is_winner(2, 8));
            assert!(is_winner(3, 7));
            assert!(!is_winner(3, 8));
        }

        // Test `generate_bet_id` function
        // #[test]
        // fn bet_id_format() {
        //     let env = mock_env();
        //     let info = MessageInfo {
        //         sender: Addr::unchecked("sender_address"),
        //         funds: vec![],
        //     };
        //     let bet_id = generate_bet_id(&mut 3000);

        //     assert!(
        //         bet_id.starts_with(&env.block.time.seconds().to_string()),
        //         "Bet ID does not start with timestamp"
        //     );

        //     assert!(
        //         bet_id.contains("sender_address"),
        //         "Bet ID does not contain sender address"
        //     );

        //     let expected_length =
        //         env.block.time.seconds().to_string().len() + 1 + "sender_address".len() + 1 + 16;
        //     assert_eq!(
        //         bet_id.len(),
        //         expected_length,
        //         "Bet ID is not the correct length"
        //     );
        // }

        #[test]
        fn test_create_transfer_msg() {
            let winner_addr = Addr::unchecked("winner");
            let amount = Uint128::new(1000);

            let transfer_msg = create_transfer_msg(winner_addr.clone(), amount);

            match transfer_msg {
                BankMsg::Send {
                    to_address,
                    amount: coins,
                } => {
                    assert_eq!(
                        to_address,
                        winner_addr.to_string(),
                        "The recipient address in the message is incorrect."
                    );

                    assert_eq!(
                        coins,
                        vec![Coin {
                            denom: "ucmdx".to_string(),
                            amount: Uint128::new(1000)
                        }],
                        "The token amount or denomination in the message is incorrect."
                    );
                }
                _ => panic!("BankMsg::Send was not created."),
            }
        }
    }

    mod bet_tests {
        use crate::{calculate_loss, calculate_prize};

        use cosmwasm_std::Uint128;

        #[test]
        fn test_calculate_prize() {
            let prediction = 3u8;
            let amount = Uint128::new(100);

            let expected_prize = Uint128::new(500);
            assert_eq!(calculate_prize(prediction, amount), expected_prize);

            let prediction = 1u8;
            let expected_prize = Uint128::new(200);
            assert_eq!(calculate_prize(prediction, amount), expected_prize);
        }

        #[test]
        fn test_calculate_loss() {
            let prediction = 3u8;
            let amount = Uint128::new(100);

            let expected_loss = Uint128::new(200);
            assert_eq!(calculate_loss(prediction, amount), expected_loss);

            let prediction = 1u8;
            let expected_loss = Uint128::new(100);
            assert_eq!(calculate_loss(prediction, amount), expected_loss);
        }
    }

    mod validations_tests {
        use crate::validations::validate_bet_amount;
        use cosmwasm_std::{coins, Addr, MessageInfo, StdError, Uint128};

        // Helper function to create MessageInfo with specified token amount
        fn create_message_info(sender: &str, amount: u128, denom: &str) -> MessageInfo {
            MessageInfo {
                sender: Addr::unchecked(sender),
                funds: coins(amount, denom),
            }
        }

        #[test]
        fn test_validate_bet_amount_greater_than_zero() {
            let amount = Uint128::zero();
            let prediction = 1u8;
            let info = create_message_info("sender", 100, "token");

            let result = validate_bet_amount(amount, prediction, &info);
            assert_eq!(
                result,
                Err(StdError::generic_err(
                    "Bet amount must be greater than zero"
                ))
            );
        }

        #[test]
        fn test_validate_bet_amount_with_insufficient_balance() {
            let amount = Uint128::from(100u128);
            let prediction = 3u8;
            let info = create_message_info("sender", 150, "token"); // Insufficient for double

            let result = validate_bet_amount(amount, prediction, &info);
            assert_eq!(
                result,
                Err(StdError::generic_err(
                    "Insufficient balance to place the bet"
                ))
            );
        }

        #[test]
        fn test_validate_bet_amount_with_sufficient_balance() {
            let amount = Uint128::from(100u128);
            let prediction = 1u8;
            let info = create_message_info("sender", 100, "token"); // Sufficient balance

            let result = validate_bet_amount(amount, prediction, &info);
            assert!(result.is_ok());
        }

        #[test]
        fn test_validate_bet_amount_with_exact_required_balance_for_prediction_3() {
            let amount = Uint128::from(100u128);
            let prediction = 3u8; // Requires double the amount
            let info = create_message_info("sender", 200, "token"); // Exactly double

            let result = validate_bet_amount(amount, prediction, &info);
            assert!(result.is_ok());
        }
    }

    // Additional modules for validations, lib, etc. can be added here in a similar pattern
}
