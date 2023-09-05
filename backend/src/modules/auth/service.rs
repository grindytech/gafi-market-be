use chrono::Utc;
use mongodb::{
	bson::doc,
	options::{FindOneAndUpdateOptions, ReturnDocument},
	Collection, Database,
};
use shared::{models, Account, BaseDocument, SocialInfo};

use crate::{
	common::utils::generate_uuid,
	modules::account::{dto::AccountDTO, service::create_account},
};

use super::dto::QueryAuth;
use std::str::FromStr;
use subxt_signer::{
	sr25519::{self, Keypair},
	SecretUri,
};

/**
 *  1. FE initialize Sign in  => 2. Backend generate Nonce => 3. Store Nonce in database
 * 2. FE Fetch the nonce => Sign the nonce => Backend get the Signature => Detech Signature => If true return access token and change nonce
 */
pub async fn update_nonce(
	address: &String,
	nonce: String,
	db: Database,
) -> Result<String, mongodb::error::Error> {
	let col: Collection<AccountDTO> = db.collection(models::account::Account::name().as_str());

	/* 	let find_options = FindOneAndUpdateOptions::builder()
	.return_document(ReturnDocument::After)
	.upsert(true)
	.build(); */
	let filter = doc! {"address":address};
	let update = doc! {
		"$set":{"nonce":nonce.clone()}
	};

	if let Ok(Some(account)) = col.find_one_and_update(filter, update, None).await {
		Ok(account.address)
	} else {
		let new_account = create_account(
			AccountDTO {
				address: address.clone(),
				balance: None,
				is_verified: None,
				name: address.to_string(),
				bio: None,
				logo_url: None,
				banner_url: None,
				updated_at: Utc::now().timestamp_millis(),
				created_at: Utc::now().timestamp_millis(),
				social: SocialInfo {
					discord: None,
					facebook: None,
					medium: None,
					twitter: None,
					web: None,
				},
				favorites: None,
				nonce: Some(nonce),
			},
			db.clone(),
		)
		.await;

		match new_account {
			Ok(account) => Ok(account),
			Err(e) => Err(e),
		}
	}
}

pub async fn get_access_token(
	params: QueryAuth,
	db: Database,
) -> Result<Option<Account>, mongodb::error::Error> {
	let address = params.address;
	let signature = params.signature;

	let uri = SecretUri::from_str("//Alice").unwrap();
	let keypair = Keypair::from_uri(&uri).unwrap();
	let message = b"Hello world!";
	let message_2 = b"idonknowwhat";
	let signature_test = keypair.sign(message);

	let public_key = keypair.public_key();

	log::info!(
		"Check success {:?}",
		sr25519::verify(&signature_test, message_2, &public_key)
	);

	let collection: Collection<Account> = db.collection(models::Account::name().as_str());
	let new_nonce = generate_uuid();
	let filter = doc! {
		"$and": [
			{"address": address},
		/* 	{"nonce": signature}, */
		],

	};
	let update = doc! {
		"$set":{"nonce":new_nonce}
	};

	if let Ok(Some(account_detail)) = collection.find_one_and_update(filter, update, None).await {
		Ok(Some(account_detail))
	} else {
		Ok(None)
	}
}
