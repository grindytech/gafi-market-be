use std::str::FromStr;

use crate::common::utils::{generate_jwt_token, verify_jwt_token};
use crate::{app_state::AppState, common::utils::verify_signature};
use actix_web::web::Data;
use dotenv::dotenv;
use shared::Config;
use subxt_signer::sr25519::{self, Signature};
use subxt_signer::{sr25519::Keypair, SecretUri};

fn signature_to_hex_string(signature: &Signature) -> String {
	let hex_string: String = signature.0.iter().map(|byte| format!("{:02X}", byte)).collect();
	hex_string
}
fn hex_string_to_signature(hex_string: &str) -> Result<Signature, &'static str> {
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
	if bytes.len() != 0 {
		return Err("Invalid signature length");
	}

	Ok(Signature(bytes.try_into().unwrap()))
}

async fn sign_test_message(message: String, config: Config) -> Signature {
	let uri = SecretUri::from_str(&config.key_pair_hash).expect("Error get Scret Key");
	let keypair = Keypair::from_uri(&uri).expect("Error get keypair");
	//get the hash keypair
	// convert message check to message check signature
	let signature_test = keypair.sign(&message.as_bytes().to_vec());
	signature_test
}

fn verify_test_signature(message: String, signature: Signature, config: Config) {
	let result = verify_signature(signature, &message, config);
	print!("Verify Signature {:?}", result)
}

#[actix_web::test]
async fn test() {
	dotenv().ok();
	let configuration = Config::init();
	print!("Not run ??");
	let test_message="Welcome to Gafi Market!\n\nClick to sign in and accept the GafiMarket Terms of Service (https://apps.gafi.network/) and Privacy Policy (https://apps.gafi.network/).\n\nThis request will not trigger a blockchain transaction or cost any gas fees.\n\nYour authentication status will reset after 24 hours.\n\nWallet address:\n0sxbdfc529688922fb5036d9439a7cd61d61114f600\n\nNonce:\ndbb29a2f-4405-4e7d-8317-424d7978d4fe".to_string();
	let signature = sign_test_message(test_message.clone(), configuration.clone()).await;

	let test = signature_to_hex_string(&signature);

	println!("Signature {:?}", test);

	verify_test_signature(test_message, signature, configuration.clone());

	//Test Generate JWT and Decode JWT:
	let jwt_token = generate_jwt_token(
		"5HC2BvrZTXc3DCxDVm6en2tn7iE8bzZnHA4gPEeM3sDL1TkW".to_string(),
		configuration.clone(),
	);
	println!("Current JWT TOken {:?} \n", jwt_token);
	let result_token = verify_jwt_token(jwt_token.unwrap(), configuration);
	println!("Result Verify JWT {:?} \n", result_token);
}
