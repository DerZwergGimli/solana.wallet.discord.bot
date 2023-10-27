#[cfg(test)]
mod test_wallet {
    use solana_wallet::wallet::Wallet;


    #[tokio::test]
    async fn test_token_accounts() {
        let mut wallet = Wallet::new(
            "https://solana-mainnet.g.alchemy.com/v2/AaKsvOkJp4LwaW08RHWRZo43ZWtYPiOD".to_string(),
            "756pfnvP3HHRx1BPwBPQwe1xBMfMWef5N9oN61Ews7np".to_string());

        // let mut wallet = Wallet::new(
        //     "https://solana-mainnet.rpc.extrnode.com".to_string(),
        //     "756pfnvP3HHRx1BPwBPQwe1xBMfMWef5N9oN61Ews7np".to_string());


        // SETUP
        wallet.load_config();
        if wallet.wallet_tokens.len() == 0 {
            // Update
            wallet.update_accounts().await;
            wallet.update_accounts_balances().await;
            wallet.update_token_names().await;
            wallet.save_config()
        }

        //Execute
        wallet.print_accounts();
        let transfers = wallet.get_and_update_signatures().await;


        for transfer in transfers {
            println!("{:?}", transfer)
        }

        wallet.save_config();


        let value = 10;
        assert_eq!(10, value);
    }

    #[tokio::test]
    async fn test_no_save() {
        let mut wallet = Wallet::new(
            "https://solana-mainnet.g.alchemy.com/v2/AaKsvOkJp4LwaW08RHWRZo43ZWtYPiOD".to_string(),
            "756pfnvP3HHRx1BPwBPQwe1xBMfMWef5N9oN61Ews7np".to_string());

        // let mut wallet = Wallet::new(
        //     "https://solana-mainnet.rpc.extrnode.com".to_string(),
        //     "756pfnvP3HHRx1BPwBPQwe1xBMfMWef5N9oN61Ews7np".to_string());


        // SETUP
        wallet.load_config();


        //Execute
        wallet.print_accounts();
        let transfers = wallet.get_and_update_signatures().await;


        for transfer in transfers {
            println!("{:?}", transfer)
        }




        let value = 10;
        assert_eq!(10, value);
    }
}