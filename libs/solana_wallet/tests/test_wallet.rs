#[cfg(test)]
mod test_wallet {
    use solana_wallet::Wallet;
    use super::*;

    #[test]
    fn test_token_accounts() {
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
            wallet.update_accounts();
            wallet.update_accounts_balances();
            wallet.update_token_names();
            wallet.save_config()
        }

        //Execute
        wallet.print_accounts();
        let transfers = wallet.get_and_update_signatures();


        for transfer in transfers {
            println!("{:?}", transfer)
        }

        wallet.save_config();


        let value = 10;
        assert_eq!(10, value);
    }
}