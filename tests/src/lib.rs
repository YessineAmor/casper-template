#[cfg(test)]
mod tests {
    use casper_engine_test_support::{Code, SessionBuilder, TestContextBuilder};
    use casper_types::{account::AccountHash, runtime_args, RuntimeArgs, U512,PublicKey};


    #[test]
    fn should_initialize_to_zero() {
    let my_account : PublicKey = PublicKey::ed25519([3u8; 32]).unwrap();
    let my_account_hash: AccountHash = my_account.to_account_hash();
    const KEY: &str = "special_value";
    const CONTRACT: &str = "tutorial";

        let mut context = TestContextBuilder::new()
            .with_public_key(my_account,my_account_hash, U512::from(500_000_000_000_000_000u64))
            .build();
        let session_code = Code::from("contract.wasm");
        let session_args = runtime_args! {
            "initial_value" => 0u64
        };
        let session = SessionBuilder::new(session_code, session_args)
            .with_address(my_account_hash)
            .with_authorization_keys(&[my_account_hash])
            .with_block_time(0)
            .build();
        context.run(session);
        let check: u64 = match context.query(my_account_hash, &[CONTRACT.to_string(), KEY.to_string()]) {
            Err(_) => panic!("Error"),
            Ok(maybe_value) => maybe_value
                .into_t()
                .unwrap_or_else(|_| panic!("{} is not expected type.", KEY)),
        };
        assert_eq!(0, check);
    }
}