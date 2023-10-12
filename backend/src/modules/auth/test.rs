use subxt_signer::{bip39::Mnemonic, sr25519};

use std::env;

use hex::ToHex;
#[actix_web::test]
async fn test() {
	let keypair = sr25519::dev::alice();
	let message = b"Hello!";

	let signature = keypair.sign(message);
	let public_key = keypair.public_key();

	println!("Signature type {:?}", hex::encode(signature.as_ref()));
	assert!(sr25519::verify(&signature, message, &public_key));

	let mut message = String::from("Hello world!");

	let args: Vec<String> = env::args().collect();
	if args.len() > 1 {
		message = args[1].clone();
	}

	println!("Message: {}", message);

	let msg = message.as_bytes();
}
