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

	/* let rng = rand::SystemRandom::new();

	let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng).unwrap();

	let key_pair = signature::Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()).unwrap();

	let peer_public_key_bytes = key_pair.public_key().as_ref();

	println!("\nPkcs8: {:?}", pkcs8_bytes.as_ref().encode_hex::<String>());
	println!(
		"\nPublic key: {:?}",
		peer_public_key_bytes.encode_hex::<String>()
	);

	let sig = key_pair.sign(msg);

	let sig_bytes = sig.as_ref();

	println!("\nSignature: {:?}", sig_bytes.encode_hex::<String>());

	let peer_public_key =
		signature::UnparsedPublicKey::new(&signature::ED25519, peer_public_key_bytes);

	let rtn = peer_public_key.verify(msg, sig.as_ref()).is_ok();

	if rtn == true {
		println!("\nMessage signature correct");
	} else {
		println!("\nMessage signature incorrect");
	} */
}
