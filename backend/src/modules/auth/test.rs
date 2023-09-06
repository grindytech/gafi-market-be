use std::str::FromStr;

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
	print!("What the hell verify {:?}", result)
}

#[actix_web::test]
async fn test() {
	dotenv().ok();
	let configuration = Config::init();
	print!("Not run ??");
	let test_message="Welcome to Gafi Market!\n\nClick to sign in and accept the GafiMarket Terms of Service (https://apps.gafi.network/) and Privacy Policy (https://apps.gafi.network/).\n\nThis request will not trigger a blockchain transaction or cost any gas fees.\n\nYour authentication status will reset after 24 hours.\n\nWallet address:\n0sxbdfc529688922fb5036d9439a7cd61d61114f600\n\nNonce:\n126829a5-99cc-4f3c-9694-dbe81d322d89".to_string();
	let signature = sign_test_message(test_message.clone(), configuration.clone()).await;

	let test_re = hex_string_to_signature("A614657C04A7C037F8377F418D400F32CABC8E301C26503B32C842DF602B8D4E46CA797C8BE5AE0C309243D12FED7CE856A012CF02B26E7781DB52A165196186");

	let test = signature_to_hex_string(&signature);

	println!("Signature {:?}", test);

	verify_test_signature(test_message, signature, configuration);
}
