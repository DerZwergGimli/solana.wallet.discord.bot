use std::fs::File;
use std::io::{Read, Write};
use std::str::FromStr;

use anyhow::{anyhow, Error};
use log::{error, info, warn};
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_transaction_status::{EncodedTransaction, UiMessage, UiTransactionEncoding};
use spl_token::instruction::TokenInstruction;

use configuration::configuration::Configuration;
use configuration::helper::write_config;

use crate::mapped_tx::MappedTX;

pub struct TxScanner {
    rpc_url: String,
    config: Configuration,
}

impl TxScanner {
    pub fn new(config: Configuration) -> Self {
        TxScanner {
            rpc_url: config.clone().rpc_url,
            config,
        }
    }


    pub fn update_config(&mut self, mapped_txs: Vec<MappedTX>) {
        if !mapped_txs.is_empty() {
            //find and update last signatures to config
            for (idx, account) in self.config.accounts.clone().into_iter().enumerate() {
                self.config.accounts[idx].last_signature = mapped_txs.clone().into_iter().filter(|tx| {
                    (tx.source_account == account.account || tx.destination_account == account.account)
                }).max_by_key(|account| account.block).unwrap().signature
            }
            write_config("config.json".to_string(), self.config.clone());
        } else {
            warn!("mapped_tx was empty or config_update is disbaled")
        }
    }

    pub async fn check(&self) -> Result<Vec<MappedTX>, Error> {
        let signatures_new = self.find_new_signatures().expect("Error finding signatures");
        let mapped_tx = self.map_transactions(signatures_new);

        info!("Checked for new transactions: {:?}", mapped_tx);
        Ok(mapped_tx)
    }


    fn find_new_signatures(&self) -> Result<Vec<String>, Result<(), Error>> {
        let client = RpcClient::new(self.rpc_url.clone());

        let mut signatures_new: Vec<String> = vec![];
        for (idx, account) in self.config.accounts.clone().into_iter().enumerate() {
            let signatures = client.get_signatures_for_address(
                &Pubkey::from_str(account.account.as_str()).unwrap(),
            );
            match signatures {
                Ok(signatures) => {
                    let index_stored_signature = signatures.clone().into_iter().position(|sign| {
                        sign.signature == account.clone().last_signature
                    }).expect("Unable to find any matching signature index");

                    for (idx, signature) in signatures.clone().into_iter().enumerate() {
                        if index_stored_signature > idx {
                            signatures_new.push(signature.signature);
                        }
                    }
                }
                Err(_) => {
                    return Err(Err(anyhow!("Error fetching signatures")));
                }
            }
        }
        Ok(signatures_new.into_iter().rev().collect())
    }

    fn map_transactions(&self, signatures_new: Vec<String>) -> Vec<MappedTX> {
        let mut mapped_tx: Vec<MappedTX> = vec![];
        let client = RpcClient::new(self.rpc_url.clone());
        signatures_new.into_iter().for_each(|sign| {
            let transaction = client.get_transaction(&Signature::from_str(sign.as_str()).unwrap(), UiTransactionEncoding::Json);

            if let Ok(tx) = transaction {
                let message = match tx.transaction.transaction.clone() {
                    EncodedTransaction::LegacyBinary(_) => { None }
                    EncodedTransaction::Binary(_, _) => { None }
                    EncodedTransaction::Json(json) => {
                        match json.message {
                            UiMessage::Parsed(_) => { None }
                            UiMessage::Raw(raw) => { Some(raw) }
                        }
                    }
                    EncodedTransaction::Accounts(_) => { None }
                };


                if let Some(msg) = message {
                    msg.instructions.into_iter().for_each(|i| {
                        if msg.account_keys[i.program_id_index as usize] == "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA" {
                            let data = bs58::decode(i.data).into_vec().unwrap();
                            let token_instruction = TokenInstruction::unpack(&data);
                            if let Ok(TokenInstruction::TransferChecked { amount, decimals }) = token_instruction {
                                mapped_tx.push({
                                    MappedTX {
                                        signature: sign.to_string(),
                                        block: tx.slot,
                                        timestamp: tx.block_time.unwrap_or_default(),
                                        source_account: msg.account_keys[i.accounts[0] as usize].clone(),
                                        destination_account: msg.account_keys[i.accounts[2] as usize].clone(),
                                        mint_send: msg.account_keys[i.accounts[1] as usize].clone(),
                                        signer: msg.account_keys[i.accounts[3] as usize].clone(),
                                        amount_send_parsed: (amount as f64 * f64::powi(10.0, -(decimals as i32))),
                                        message: "Token Transfer".to_string(),
                                    }
                                });
                            }
                        }
                    })
                }
            }
        });
        mapped_tx
    }
}
