use shared::{db, Config};
use subxt_signer::{bip39::Mnemonic, sr25519};

use std::env;

use hex::ToHex;

use crate::common::utils::{generate_message_sign_in, generate_uuid};

/**
 *  Input: Address , Signature
 * Flow: 1. Get the sign message
 */
async fn test_flow_verify_signature() -> bool {
	// random nonce use uuid library more complex
	let nonce = generate_uuid();
	println!("Nonce Value {:?}", nonce);
	// Test Keypair Dev
	let keypair = sr25519::dev::alice();
	let public_key = keypair.public_key();

	let address = hex::encode(&public_key);
	println!("Address  {:?}", address);
	// generate message
	let generate_message = generate_message_sign_in(&address, &nonce);
	let signature = keypair.sign(generate_message.as_bytes());

	assert!(sr25519::verify(&signature, generate_message, &public_key));
	true
}

#[actix_web::test]
async fn test() {
	dotenv::dotenv().ok();
	let configuration = Config::init();

	let result = test_flow_verify_signature().await;
}
