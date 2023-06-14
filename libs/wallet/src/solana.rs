use std::str::FromStr;

use log::info;
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;

use configuration::configuration::Configuration;

use crate::accounts_details::AccountDetails;

pub struct Wallet {
    rpc_url: String,
    config: Configuration,
}

impl Wallet {
    pub fn new(config: Configuration) -> Self {
        Wallet {
            rpc_url: "https://solana-mainnet.rpc.extrnode.com".to_string(),
            config,
        }
    }

    pub async fn get_token_amounts(&self) -> Vec<AccountDetails> {
        let mut account_details: Vec<AccountDetails> = vec![];
        let client = RpcClient::new(self.rpc_url.clone());

        for account in self.config.clone().accounts.into_iter() {
            let account_data = client.get_token_account(&Pubkey::from_str(account.account.as_str()).unwrap()).unwrap();

            account_details.push({
                AccountDetails {
                    account: account.clone().account,
                    mint: account.mint,
                    amount: account_data.unwrap().token_amount.ui_amount.unwrap_or_default(),
                }
            });
        }
        info!("Token account details fetched!");
        account_details
    }
}
