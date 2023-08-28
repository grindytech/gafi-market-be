use super::TokenPayload;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use mongodb::bson::doc;
use rand::Rng;
use std::env::var;

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

pub fn generate_random_six_digit_number() -> u32 {
	let mut rng = rand::thread_rng();
	rng.gen_range(100_000..999_999)
}

pub fn generate_jwt_token(
	address: String,
	sub: String,
) -> Result<String, jsonwebtoken::errors::Error> {
	// Define the current timestamp
	let current_timestamp = Utc::now().timestamp() as usize;

	// Define the payload data
	let payload = TokenPayload {
		address,
		sub,
		iat: current_timestamp,
		exp: current_timestamp + 3600, // Token expires in 1 hour
	};
	let secret_key = std::env::var("JWT_TOKEN_SECRET").expect("JWT_TOKEN_SECRET must be set");
	let token = encode(
		&Header::new(Algorithm::HS256),
		&payload,
		&EncodingKey::from_secret(secret_key.as_ref()),
	)?;

	Ok(token)
}
