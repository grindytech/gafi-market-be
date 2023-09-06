use std::str::FromStr;

use crate::app_state::AppState;

use super::TokenPayload;
use actix_web::web::Data;
use chrono::Utc;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use mongodb::bson::doc;
use shared::Config;
use subxt_signer::{
	sr25519::{self, Keypair, Signature},
	SecretUri,
};
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
pub fn generate_message_sign_in(wallet_address: &String, nonce: &String) -> String {
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
pub fn hex_string_to_signature(hex_string: &str) -> Result<Signature, &'static str> {
	// Check if the hex string has an even number of characters (2 characters per byte)
	if hex_string.len() % 2 != 0 {
		return Err("Invalid hex string length");
	}

	// Create a vector to hold the bytes
	let mut bytes = Vec::new();

	// Iterate over pairs of characters in the hex string and parse them as bytes
	for i in 0..hex_string.len() / 2 {
		let byte_str = &hex_string[i * 2..(i * 2) + 2];
		if let Ok(byte) = u8::from_str_radix(byte_str, 16) {
			bytes.push(byte);
		} else {
			return Err("Invalid hex string format");
		}
	}

	// Check if the parsed bytes form a valid signature

	Ok(Signature(bytes.try_into().unwrap()))
}

pub fn verify_signature(signature: Signature, message: &String, config: Config) -> bool {
	let uri = SecretUri::from_str(&config.key_pair_hash).unwrap();
	let keypair = Keypair::from_uri(&uri).unwrap();

	let public_key = keypair.public_key();

	/* 	log::info!(
		"Check success {:?}",
		sr25519::verify(&signature, message_2, &public_key)
	); */
	sr25519::verify(&signature, message, &public_key)
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
		exp: current_timestamp + app.config.jwt_expire_time, // Token expires in 1 hour
	};

	let token = encode(
		&Header::new(Algorithm::HS256),
		&payload,
		&EncodingKey::from_secret(app.config.jwt_secret_key.as_ref()),
	);
	match token {
		Ok(token) => Ok(token),
		Err(e) => Err(e),
	}
}
