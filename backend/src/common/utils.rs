use crate::app_state::AppState;

use super::TokenPayload;
use actix_web::web::Data;
use chrono::Utc;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use mongodb::bson::doc;
use uuid::Uuid;

pub async fn get_total_page(number_items: usize, size: u64) -> u64 {
	(number_items as f64 / size as f64).ceil() as u64
}

pub async fn get_filter_option(
	order_by: String,
	des: bool,
) -> Option<mongodb::options::FindOptions> {
	let sort = if des { 1 } else { -1 };
	let sort = doc! { order_by:sort };
	let mut find_options = mongodb::options::FindOptions::default();
	find_options.sort = Some(sort);
	Some(find_options)
}

pub fn generate_uuid() -> String {
	let uuid = Uuid::new_v4();
	uuid.to_string()
}
pub fn generate_message_sign_in(wallet_address: &str, nonce: &str) -> String {
	let template = format!(
        "Welcome to Gafi Market!\n\
         \n\
         Click to sign in and accept the GafiMarket Terms of Service (https://apps.gafi.network/) and Privacy Policy (https://apps.gafi.network/).\n\
         \n\
         This request will not trigger a blockchain transaction or cost any gas fees.\n\
         \n\
         Your authentication status will reset after 24 hours.\n\
         \n\
         Wallet address:\n\
         {}\n\
         \n\
         Nonce:\n\
         {}",
        wallet_address, nonce
    );

	template
}

pub fn generate_jwt_token(
	address: String,
	app: Data<AppState>,
) -> Result<String, jsonwebtoken::errors::Error> {
	// Define the current timestamp
	let current_timestamp = Utc::now().timestamp_millis();

	// Define the payload data
	let payload = TokenPayload {
		address,
		iat: current_timestamp,
		exp: current_timestamp + app.config.expire_time, // Token expires in 1 hour
	};

	let token = encode(
		&Header::new(Algorithm::HS256),
		&payload,
		&EncodingKey::from_secret(app.config.secret_key.as_ref()),
	)?;

	Ok(token)
}
