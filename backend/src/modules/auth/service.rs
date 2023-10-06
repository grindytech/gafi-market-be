use std::str::FromStr;

use crate::{
	app_state::AppState,
	common::utils::{generate_message_sign_in, generate_uuid},
	modules::account::{dto::AccountDTO, service::create_account},
};
use actix_web::web::Data;
use chrono::Utc;
use mongodb::{bson::doc, Collection, Database};

use shared::{models, utils::vec_to_array_64, Account, BaseDocument, SocialInfo};

use subxt_signer::sr25519::{PublicKey, Signature};

use super::dto::QueryAuth;

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

/**
 *
 * Check current nonce from the address
 * compare signature from this
 */

pub async fn get_access_token(
	params: QueryAuth,
	app: Data<AppState>,
) -> Result<Option<Account>, mongodb::error::Error> {
	let collection: Collection<Account> =
		app.db.clone().collection(models::Account::name().as_str());

	let address = params.address;
	let signature = params.signature;

	log::info!("Address : {:?}", &address);
	log::info!("Signature : {:?}", &signature);

	let mut nonce_value: String = "".to_string();
	let filter = doc! {
		"$and": [
			{"address": &address},
		],

	};

	if let Ok(Some(account)) = collection.find_one(filter.clone(), None).await {
		match account.nonce {
			Some(value) => nonce_value = value,
			None => (),
		}
	} else {
		return Ok(None);
	}

	let message = generate_message_sign_in(&address, &nonce_value);

	// decodate address from public account 32
	let public_key = subxt::utils::AccountId32::from_str(&address).unwrap();

	let sign = &signature[2..].to_string();

	let signature = hex::decode(&sign).unwrap();
	/* log::info!("Current Signature Decode {:?}", signature.len()); */
	let log_fe = vec_to_array_64(signature);

	let result =
		subxt_signer::sr25519::verify(&Signature(log_fe), message, &PublicKey(public_key.0));

	if result == false {
		return Ok(None);
	};
	let new_nonce = generate_uuid();

	let update = doc! {
		"$set":{"nonce":new_nonce}
	};

	if let Ok(Some(account_detail)) = collection.find_one_and_update(filter, update, None).await {
		Ok(Some(account_detail))
	} else {
		Ok(None)
	}
}
