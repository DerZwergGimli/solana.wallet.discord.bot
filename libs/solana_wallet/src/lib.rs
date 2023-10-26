mod token_lists;

use std::fmt;
use std::fs::File;
use std::str::FromStr;
use std::time::Duration;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use serde_json::json;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_client::GetConfirmedSignaturesForAddress2Config;
use solana_client::rpc_config::RpcTransactionConfig;
use solana_client::rpc_request::TokenAccountsFilter;
use solana_client::rpc_response::RpcConfirmedTransactionStatusWithSignature;
use solana_program::bpf_loader_upgradeable::close;
use solana_program::pubkey::Pubkey;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signature::Signature;
use solana_transaction_status::{EncodedTransaction, UiMessage, UiRawMessage, UiTransactionEncoding};
use spl_token::instruction::TokenInstruction;
use tabled::{Table, Tabled};
use tokio::join;
use crate::token_lists::token_list_solflare::{TokenListSolflare};
use crate::token_lists::token_list_staratlas::{TokenListStarAtlas, TokenListStarAtlasElement};


const TOKEN_PROGRAM: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
const CONFIG_FILE: &str = "wallet_config.json";

#[derive(Tabled, Debug, Clone, Serialize, Deserialize)]
struct WalletTokenInfo {
    name: String,
    symbol: String,
}

#[derive(Tabled, Debug, Clone, Serialize, Deserialize)]
pub struct WalletToken {
    last_signature: String,
    info: WalletTokenInfo,
    account: String,
    mint: String,
    decimals: u8,
    amount: u64,
}

#[derive(Tabled, Debug, Clone)]
pub struct WalletTransaction {
    instruction: String,
    signature: String,
    account: String,
    mint: String,
    decimals: u8,
    amount: u64,
    is_incoming: bool,

    from: String,
    to: String,

}

pub struct Wallet {
    client: solana_client::rpc_client::RpcClient,
    pub wallet_address: Pubkey,
    pub wallet_tokens: Vec<WalletToken>,
}

impl Wallet {
    pub fn new(url: String, wallet: String) -> Self {
        Self {
            client: solana_client::rpc_client::RpcClient::new_with_timeout(url.clone(), Duration::from_secs(3)),
            wallet_address: Pubkey::from_str(wallet.as_str()).unwrap(),
            wallet_tokens: vec![],
        }
    }

    pub fn load_config(&mut self) {
        match File::open(CONFIG_FILE) {
            Ok(file) => {
                self.wallet_tokens = serde_json::from_reader(file).unwrap();
                info!("Wallet config has been loaded from store!")
            }
            _ => { warn!("No config could be loaded!") }
        }
    }
    pub fn save_config(&self) {
        let file = File::create(CONFIG_FILE).unwrap();
        serde_json::to_writer_pretty(file, &self.wallet_tokens).unwrap();
    }
    pub fn update_accounts(&mut self) {
        let filter = TokenAccountsFilter::ProgramId(Pubkey::from_str(TOKEN_PROGRAM).unwrap());

        match self.client.get_token_accounts_by_owner(&self.wallet_address, filter) {
            Ok(accounts) => {
                for account in accounts {
                    self.wallet_tokens.push(WalletToken {
                        last_signature: "".to_string(),
                        info: WalletTokenInfo {
                            name: "".to_string(),
                            symbol: "".to_string(),
                        },
                        account: account.pubkey.clone(),
                        mint: "".to_string(),
                        amount: 0,
                        decimals: 0,
                    })
                }
            }
            Err(err) => {
                error!("Error while getting accounts: {}", err)
            }
        }
    }

    pub fn update_accounts_balances(&mut self) {
        for (wallet_token_index, wallet_token) in self.wallet_tokens.clone().into_iter().enumerate() {
            let ui_token_account = self.client.get_token_account(&Pubkey::from_str(wallet_token.account.as_str()).unwrap()).unwrap().unwrap();
            self.wallet_tokens[wallet_token_index].decimals = ui_token_account.token_amount.decimals;
            self.wallet_tokens[wallet_token_index].amount = ui_token_account.token_amount.amount.parse::<u64>().unwrap();
            self.wallet_tokens[wallet_token_index].mint = ui_token_account.mint
        }
    }

    pub fn update_token_names(&mut self) {
        let token_list_solflare: TokenListSolflare = reqwest::blocking::get("https://cdn.jsdelivr.net/gh/solflare-wallet/token-list@latest/solana-tokenlist.json").unwrap().json().unwrap();
        let token_list_staratlas: TokenListStarAtlas = reqwest::blocking::get("https://galaxy.staratlas.com/nfts").unwrap().json().unwrap();

        for (wallet_token_index, wallet_token) in self.wallet_tokens.clone().into_iter().enumerate() {
            self.wallet_tokens[wallet_token_index].info = match token_list_solflare.tokens.clone().into_iter().find(|t| t.address == wallet_token.mint) {
                None => {
                    match token_list_staratlas.clone().into_iter().find(|t| t.mint == wallet_token.mint) {
                        None => {
                            WalletTokenInfo {
                                name: "".to_string(),
                                symbol: "".to_string(),

                            }
                        }
                        Some(token_sa) => {
                            WalletTokenInfo {
                                name: token_sa.name,
                                symbol: token_sa.symbol,

                            }
                        }
                    }
                }
                Some(token_sol) => {
                    WalletTokenInfo {
                        name: token_sol.name,
                        symbol: token_sol.symbol,
                    }
                }
            }
        }
    }

    pub fn get_and_update_signatures(&mut self) -> Vec<WalletTransaction> {
        let mut transactions = vec![];

        for (token_index, mut token) in self.wallet_tokens.clone().into_iter().enumerate() {
            let config = if token.last_signature.len() > 0 {
                GetConfirmedSignaturesForAddress2Config {
                    before: None,
                    until: Some(Signature::from_str(token.last_signature.clone().as_str()).unwrap()),
                    limit: Some(100),
                    commitment: Some(CommitmentConfig::confirmed()),
                }
            } else {
                GetConfirmedSignaturesForAddress2Config {
                    before: None,
                    until: None,
                    limit: Some(1),
                    commitment: Some(CommitmentConfig::confirmed()),
                }
            };

            let mut signatures = self.client.get_signatures_for_address_with_config(&Pubkey::from_str(token.clone().account.as_str()).unwrap(), config).unwrap();
            signatures.reverse();
            for signature in signatures {
                if let Ok(transaction) = self.client.get_transaction_with_config(
                    &Signature::from_str(signature.signature.as_str()).unwrap(),
                    RpcTransactionConfig {
                        encoding: Some(UiTransactionEncoding::Json),
                        commitment: None,
                        max_supported_transaction_version: Some(0),
                    },
                ) {
                    let message = match transaction.transaction.transaction.clone() {
                        EncodedTransaction::Json(json) => {
                            match json.message {
                                UiMessage::Raw(raw) => Some(raw),
                                _ => { None }
                            }
                        }
                        _ => { None }
                    };

                    match message {
                        None => { panic!("Error no message was able to be decoded!") }
                        Some(mesage) => {
                            for instruction in mesage.instructions {
                                if mesage.account_keys[instruction.program_id_index as usize] == TOKEN_PROGRAM {
                                    let temp = bs58::decode(instruction.data).into_vec().unwrap();
                                    let token_instruction = TokenInstruction::unpack(&temp).unwrap();

                                    match token_instruction {
                                        TokenInstruction::InitializeMint { .. } => {
                                            Self::transaction_push_default("InitializeMint", &mut transactions, token.clone(), signature.clone());
                                        }
                                        TokenInstruction::InitializeAccount => {
                                            Self::transaction_push_default("InitializeAccount", &mut transactions, token.clone(), signature.clone());
                                        }
                                        TokenInstruction::InitializeMultisig { .. } => {
                                            Self::transaction_push_default("InitializeMultisig", &mut transactions, token.clone(), signature.clone());
                                        }
                                        TokenInstruction::Transfer { amount } => {
                                            transactions.push(WalletTransaction {
                                                instruction: "WalletTransaction".to_string(),
                                                signature: signature.signature.to_string(),
                                                account: token.account.to_string(),
                                                mint: token.mint.to_string(),
                                                decimals: token.decimals,
                                                amount,
                                                from: mesage.account_keys[instruction.accounts[1] as usize].clone(),
                                                to: mesage.account_keys[instruction.accounts[2] as usize].clone(),
                                                is_incoming: mesage.account_keys[instruction.accounts[2] as usize].clone() == self.wallet_address.to_string(),
                                            })
                                        }
                                        TokenInstruction::Approve { .. } => {
                                            Self::transaction_push_default("Approve", &mut transactions, token.clone(), signature.clone());
                                        }
                                        TokenInstruction::Revoke => {
                                            Self::transaction_push_default("Revoke", &mut transactions, token.clone(), signature.clone());
                                        }
                                        TokenInstruction::SetAuthority { .. } => {
                                            Self::transaction_push_default("SetAuthority", &mut transactions, token.clone(), signature.clone());
                                        }
                                        TokenInstruction::MintTo { .. } => {
                                            Self::transaction_push_default("MintTo", &mut transactions, token.clone(), signature.clone());
                                        }
                                        TokenInstruction::Burn { .. } => {
                                            Self::transaction_push_default("Burn", &mut transactions, token.clone(), signature.clone());
                                        }
                                        TokenInstruction::CloseAccount => {
                                            Self::transaction_push_default("CloseAccount", &mut transactions, token.clone(), signature.clone());
                                        }
                                        TokenInstruction::FreezeAccount => {
                                            Self::transaction_push_default("FreezeAccount", &mut transactions, token.clone(), signature.clone());
                                        }
                                        TokenInstruction::ThawAccount => {
                                            Self::transaction_push_default("ThawAccount", &mut transactions, token.clone(), signature.clone());
                                        }
                                        TokenInstruction::TransferChecked { amount, decimals } => {
                                            transactions.push(WalletTransaction {
                                                instruction: "TransferChecked".to_string(),
                                                signature: signature.signature.to_string(),
                                                account: token.account.to_string(),
                                                mint: token.mint.to_string(),
                                                decimals,
                                                amount,
                                                from: mesage.account_keys[instruction.accounts[1] as usize].clone(),
                                                to: mesage.account_keys[instruction.accounts[2] as usize].clone(),
                                                is_incoming: mesage.account_keys[instruction.accounts[2] as usize].clone() == self.wallet_address.to_string(),
                                            })
                                        }
                                        TokenInstruction::ApproveChecked { .. } => {
                                            Self::transaction_push_default("ApproveChecked", &mut transactions, token.clone(), signature.clone());
                                        }
                                        TokenInstruction::MintToChecked { .. } => {
                                            Self::transaction_push_default("MintToChecked", &mut transactions, token.clone(), signature.clone());
                                        }
                                        TokenInstruction::BurnChecked { .. } => {
                                            Self::transaction_push_default("BurnChecked", &mut transactions, token.clone(), signature.clone());
                                        }
                                        TokenInstruction::InitializeAccount2 { .. } => {
                                            Self::transaction_push_default("InitializeAccount2", &mut transactions, token.clone(), signature.clone());
                                        }
                                        TokenInstruction::SyncNative => {
                                            Self::transaction_push_default("SyncNative", &mut transactions, token.clone(), signature.clone());
                                        }
                                        TokenInstruction::InitializeAccount3 { .. } => {
                                            Self::transaction_push_default("InitializeAccount3", &mut transactions, token.clone(), signature.clone());
                                        }
                                        TokenInstruction::InitializeMultisig2 { .. } => {
                                            Self::transaction_push_default("InitializeMultisig2", &mut transactions, token.clone(), signature.clone());
                                        }
                                        TokenInstruction::InitializeMint2 { .. } => {
                                            Self::transaction_push_default("InitializeMint2", &mut transactions, token.clone(), signature.clone());
                                        }
                                        TokenInstruction::GetAccountDataSize => {
                                            Self::transaction_push_default("GetAccountDataSize", &mut transactions, token.clone(), signature.clone());
                                        }
                                        TokenInstruction::InitializeImmutableOwner => {
                                            Self::transaction_push_default("InitializeImmutableOwner", &mut transactions, token.clone(), signature.clone());
                                        }
                                        TokenInstruction::AmountToUiAmount { .. } => {
                                            Self::transaction_push_default("AmountToUiAmount", &mut transactions, token.clone(), signature.clone());
                                        }
                                        TokenInstruction::UiAmountToAmount { .. } => {
                                            Self::transaction_push_default("UiAmountToAmount", &mut transactions, token.clone(), signature.clone());
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self.wallet_tokens[token_index].last_signature = signature.signature.to_string();
                }
            }
        }
        transactions
    }

    fn transaction_push_default(name: &str, transactions: &mut Vec<WalletTransaction>, token: WalletToken, signature: RpcConfirmedTransactionStatusWithSignature) {
        transactions.push(WalletTransaction {
            instruction: name.to_string(),
            signature: signature.signature.to_string(),
            account: token.account.to_string(),
            mint: token.mint.to_string(),
            decimals: 0,
            amount: 0,
            from: "".to_string(),
            to: "".to_string(),
            is_incoming: false,
        })
    }

    pub fn print_accounts(&self) {
        match self.wallet_tokens.len() {
            0 => { println!("Token-Accounts is empty!") }
            _ => {
                println!("{}", Table::new(self.wallet_tokens.clone()).to_string());
            }
        }
    }
}


impl fmt::Display for WalletTokenInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Define how you want to format the object here
        write!(f, "{} - {}", self.name, self.symbol)
    }
}

