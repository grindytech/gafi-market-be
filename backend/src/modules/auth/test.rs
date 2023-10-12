use shared::{db, Config};
use subxt_signer::{bip39::Mnemonic, sr25519};

use std::env;

use hex::ToHex;

use crate::common;

#[tokio::test]
async fn test_flow_verify_signature() {
	dotenv::dotenv().ok();
	let configuration = Config::init();
	// random nonce use uuid library more complex
	let nonce = common::utils::generate_uuid();
	println!("Nonce Value {:?}", nonce);
	// Test Keypair Dev
	let keypair = sr25519::dev::alice();
	let public_key = keypair.public_key();

	let address = hex::encode(&public_key);
	println!("Address  {:?}", address);
	// generate message
	let generate_message = common::utils::generate_message_sign_in(&address, &nonce);
	let signature = keypair.sign(generate_message.as_bytes());

	assert!(sr25519::verify(&signature, generate_message, &public_key));
}
