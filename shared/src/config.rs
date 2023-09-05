use std::env::var;
#[derive(Debug, Clone)]
pub struct Config {
	pub mongodb_uri: String,
	pub mongodb_db_name: String,
	pub rpc: String,
	pub start_block: u32,
	pub chain_decimal: u32,
	pub secret_key: String,
}

impl Config {
	pub fn init() -> Config {
		let rpc = var("RPC").unwrap_or("wss://rpc-testnet.gafi.network:443".to_string());
		let start_block: u32 = var("START_BLOCK").unwrap_or("0".to_string()).parse().unwrap();
		let mongodb_uri = std::env::var("MONGODB_URI").expect("MONGODB_URI must be set");
		let mongodb_db_name =
			std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set");
		let chain_decimal = var("CHAIN_DECIMAL").unwrap_or("18".to_string()).parse().unwrap();
		let secret_key = std::env::var("JWT_TOKEN_SECRET").expect("JWT_TOKEN_SECRET must be set");
		Config {
			mongodb_uri,
			mongodb_db_name,
			rpc,
			start_block,
			chain_decimal,
			secret_key,
		}
	}
}
