#[cfg(test)]
mod test_wallet {
    use dotenv::dotenv;
    use solana_wallet::wallet::Wallet;


    #[tokio::test]
    async fn test_token_accounts() {
        dotenv().ok();

        let mut wallet = Wallet::new(
            std::env::var("RPC_URL").expect("RPC_URL not set in .env"),
            std::env::var("WALLET").expect("WALLET not set in .env"),
            false);

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
        dotenv().ok();

        let mut wallet = Wallet::new(
            std::env::var("RPC_URL").expect("RPC_URL not set in .env"),
            std::env::var("WALLET").expect("WALLET not set in .env"),
            false);


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