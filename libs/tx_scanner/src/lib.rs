pub mod tx_scanner;
pub mod mapped_tx;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.

    use assert_json::assert_json;

    use crate::tx_scanner::TxScanner;

    use super::*;

    #[test]
    fn scan() {
        let json_config = r#"
        {
          "accounts": [
            {
              "address": "AsnhnzZpAQ5JN8jM9j194y4TrpxeeW8M93KdG8Rj1ARv",
              "last_signature": "4wju4cTr7nuQu9fdNcDScpo3jhm9imLHMtE1Jb8rqiUWGDYJvCRWQs4qQ2emxyKG9Gp1NqmNJET5RzZhkDt6f8wk"
            }
          ]
        }
        "#;

        let mut scanner = TxScanner::new();
        scanner.load_config();

        assert_json!(json_config, scanner);
    }
}
