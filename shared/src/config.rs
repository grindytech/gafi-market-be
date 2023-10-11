use std::env::var;
#[derive(Debug, Clone)]
pub struct Config {
	pub mongodb_uri: String,
	pub mongodb_db_name: String,
	pub rpc: String,
	pub start_block: u32,
	pub chain_decimal: u32,
	pub jwt_access_key: String,
	pub jwt_access_time: i64,
	pub jwt_refresh_key: String,
	pub jwt_refresh_time: i64,
	pub frontend_link: String,
}

impl Config {
	pub fn init() -> Config {
		let rpc = var("RPC").unwrap_or("wss://rpc-testnet.gafi.network:443".to_string());
		let start_block: u32 = var("START_BLOCK").unwrap_or("0".to_string()).parse().unwrap();
		let mongodb_uri = var("MONGODB_URI").expect("MONGODB_URI must be set");
		let mongodb_db_name = var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set");
		let chain_decimal = var("CHAIN_DECIMAL").unwrap_or("12".to_string()).parse().unwrap();
		let jwt_access_key = var("JWT_ACCESS_SECRET").expect("JWT_ACCESS_SECRET must be set");
		let jwt_access_time = var("JWT_ACCESS_TIME").unwrap_or("3600".to_string()).parse().unwrap();

		let jwt_refresh_key = var("JWT_REFRESH_SECRET").expect("JWT_REFRESH_SECRET must be set");
		let jwt_refresh_time =
			var("JWT_REFRESH_TIME").unwrap_or("86400".to_string()).parse().unwrap();

		let frontend_link = var("FRONTEND_APP").expect("Frontend link Must to set");
		Config {
			mongodb_uri,
			mongodb_db_name,
			rpc,
			start_block,
			chain_decimal,
			jwt_access_key,
			jwt_access_time,
			jwt_refresh_key,
			jwt_refresh_time,
			frontend_link,
		}
	}
}
