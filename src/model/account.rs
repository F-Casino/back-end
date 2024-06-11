use solana_sdk::pubkey::Pubkey;

pub struct Account {
    pub public_key: String,
    pub display_name: String,
}

impl Account {
    fn new(public_key: Pubkey, display_name: String) -> Account {
        Account {
            public_key: public_key.to_string(),
            display_name,
        }
    }
}
