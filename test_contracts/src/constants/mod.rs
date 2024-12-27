// test_contracts/src/constants/mod.rs

/// Re-export the `wallets` module.
pub mod wallets;

/// A submodule to manage environment variables and other constants.
pub mod env_vars {
    use std::env;

    /// A struct to hold the relevant environment variables.
    pub struct EnvVars {
        pub rpc_url: String,
        pub erc20aton_address: String,
        pub engine_address: String,
        pub whale_private_key: String,
        pub chain_id: u64,
    }

    /// Reads and returns the environment variables in a single struct.
    pub fn get_env_vars() -> EnvVars {
        let rpc_url = env::var("RPC_URL").unwrap_or_else(|_| "http://127.0.0.1:8547".into());
        let erc20aton_address = env::var("ERC20ATON_ADDRESS")
            .unwrap_or_else(|_| "http://127.0.0.1:8547".into());
        let engine_address = env::var("ENGINE_ADDRESS")
            .unwrap_or_else(|_| "http://127.0.0.1:8547".into());
        let whale_private_key = env::var("PRIVATE_KEY")
            .expect("Por favor, configura la variable de entorno PRIVATE_KEY");
        let chain_id = env::var("CHAIN_ID")
            .unwrap_or_else(|_| "412346".to_string())
            .parse::<u64>()
            .expect("CHAIN_ID is not a valid u64");

        

        EnvVars {
            rpc_url,
            erc20aton_address,
            engine_address,
            whale_private_key,
            chain_id,
        }
    }
}
