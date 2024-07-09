use solana_sdk::pubkey::Pubkey;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BetKind {
    Fish,
    Prawn,
    Crab,
    Cock,
    Calabash,
    Tiger,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BetInfo {
    pub user_public_key: Pubkey,
    pub amount: u64,
    pub kind: BetKind,
}
