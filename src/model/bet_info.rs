use sailfish::TemplateOnce;
use solana_sdk::pubkey::Pubkey;
use serde::{Deserialize, Serialize};
use super::bet_kind::BetKind;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BetInfo {
    pub user_public_key: String,
    pub amount: u64,
    pub kind: BetKind,
}
