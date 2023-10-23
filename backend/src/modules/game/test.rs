use std::str::FromStr;

use shared::utils::vec_to_array;
use subxt::utils::AccountId32;

#[tokio::test]
async fn test_find_collections() {
	let (mut db_process, db) = shared::tests::utils::get_test_db(60000).await;

	let _ = db_process.kill();
}

#[tokio::test]
async fn decode_address() {
	let encoded_string = "40bd0488c36036a0ca2d4d10e9d031de6248796f6dde1e8991f7bf248fbccf47";

	// Decode the hexadecimal string
	let decoded_bytes = hex::decode(encoded_string).expect("Failed to decode hex string");

	// Convert the byte slice to a regular string
	let original_string = AccountId32(vec_to_array(decoded_bytes));

	println!("Original string: {}", original_string);
}

#[tokio::test]
async fn endcode_address() {
	let address = "5DXb57gvDof4GYjhKSmzKiBRX1pWhKZ3ysASgaRGRTTK9BJw";
	let encoded_string = "40bd0488c36036a0ca2d4d10e9d031de6248796f6dde1e8991f7bf248fbccf47";

	let public_key = subxt::utils::AccountId32::from_str(address).unwrap();
	let encode_value = hex::encode(public_key);
	println!("Vaalue : {}", encode_value);
}
