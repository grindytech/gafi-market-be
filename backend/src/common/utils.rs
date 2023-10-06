use std::str::FromStr;

use crate::{app_state::AppState, common::ErrorResponse};

use super::TokenPayload;
use actix_web::{error::ErrorUnauthorized, web::Data};
use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use shared::Config;
use uuid::Uuid;

pub async fn get_total_page(number_items: usize, size: u64) -> u64 {
	(number_items as f64 / size as f64).ceil() as u64
}

pub fn generate_uuid() -> String {
	let uuid = Uuid::new_v4();
	uuid.to_string()
}
pub fn generate_message_sign_in(wallet_address: &String, nonce: &String) -> String {
	let template = format!(
        "<Bytes>Welcome to Gafi Market!\n\
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
         {}</Bytes>",
        wallet_address, nonce
    );

	template
}

pub fn generate_jwt_token(
	address: String,
	config: Config,
) -> Result<String, jsonwebtoken::errors::Error> {
	// Define the current timestamp
	let current_timestamp = Utc::now().timestamp_millis();

	// Define the payload data
	let payload = TokenPayload {
		address,
		iat: current_timestamp,
		exp: current_timestamp + config.jwt_expire_time, // Token expires in 1 hour
	};

	let token = encode(
		&Header::new(Algorithm::HS256),
		&payload,
		&EncodingKey::from_secret(config.jwt_secret_key.as_ref()),
	);
	match token {
		Ok(token) => Ok(token),
		Err(e) => Err(e),
	}
}

pub fn verify_jwt_token(
	token: String,
	config: Config,
) -> Result<TokenPayload, jsonwebtoken::errors::Error> {
	let claims = match decode::<TokenPayload>(
		&token,
		&DecodingKey::from_secret(config.jwt_secret_key.as_ref()),
		&Validation::new(Algorithm::HS256),
	) {
		Ok(c) => Ok(c.claims),
		Err(e) => {
			return Err(e);
		},
	};
	claims
}
