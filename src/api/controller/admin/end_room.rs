use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

use axum::extract::State;
use axum_extra::extract::CookieJar;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey, signature::Keypair, signer::Signer, system_instruction,
    transaction::Transaction,
};

use crate::config;
use crate::model::bet_kind::BetKind;
use crate::AppState;
use crate::Error;
use crate::Result;

pub async fn end_room(State(state): State<Arc<AppState>>, jar: CookieJar) -> Result<()> {
    if jar.get("token").is_none() {
        return Err(Error::Unauthorized {
            message: "Invalid token".to_string(),
        });
    };

    match state.room.lock() {
        Ok(mut room) => {
            let bet_infos = match room.as_ref() {
                Some(room) => room.bet_infos.clone(),
                None => return Err(anyhow::anyhow!("Room not found").into()),
            };

            let mut receiver_amount = HashMap::new();

            let result_bet_kind = [BetKind::Fish, BetKind::random(), BetKind::random()];
            for bet in bet_infos.iter() {
                if !result_bet_kind.contains(&bet.kind) {
                    continue;
                }

                *receiver_amount
                    .entry(bet.user_public_key.clone())
                    .or_insert(0) += bet.amount;
            }

            for (pub_key, coin) in receiver_amount.iter_mut() {
                *coin += bet_infos
                    .iter()
                    .filter(|bet| {
                        &bet.user_public_key == pub_key && result_bet_kind.contains(&bet.kind)
                    })
                    .map(|bet| bet.amount)
                    .sum::<u64>();
            }

            let client = RpcClient::new(config::SOLANA_ENDPOINT);
            let admin_key_pair = Keypair::from_base58_string(config::ADMIN_PRIVATE_KEY);

            for (receiver_pub_key, amount) in receiver_amount {
                let receiver_pub_key = Pubkey::from_str(&receiver_pub_key)
                    .map_err(|error| anyhow::anyhow!("Invalid pub key"))?;
                let transfer_instruction = system_instruction::transfer(
                    &admin_key_pair.pubkey(),
                    &receiver_pub_key,
                    amount,
                );

                let transaction = Transaction::new_signed_with_payer(
                    &[transfer_instruction],
                    Some(&admin_key_pair.pubkey()),
                    &[&admin_key_pair],
                    client.get_latest_blockhash().unwrap(),
                );

                let signature = client
                    .send_and_confirm_transaction(&transaction)
                    .map_err(|error| anyhow::anyhow!(error.to_string()))?;

                tracing::info!("Transfer to {} with amount {}", receiver_pub_key, amount);
            }

            *room = None;
        }
        Err(error) => return Err(anyhow::anyhow!("Failed to lock room").into()),
    };

    Ok(())
}
