use actix_web::web::Data;
use chrono::Utc;
use mongodb::{bson::doc, Collection, Database};
use shared::{models, Account, BaseDocument, SocialInfo};

use crate::{
	app_state::AppState,
	common::utils::{generate_uuid, hex_string_to_signature, verify_signature},
	modules::account::{dto::AccountDTO, service::create_account},
};

use super::dto::QueryAuth;

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
	app: Data<AppState>,
) -> Result<Option<Account>, mongodb::error::Error> {
	let address = params.address;
	let message = params.message;
	let signature = hex_string_to_signature(&params.signature).unwrap();

	let result = verify_signature(signature, &message, app.config.clone());
	if result == false {
		return Ok(None);
	}
	let collection: Collection<Account> =
		app.db.clone().collection(models::Account::name().as_str());
	let new_nonce = generate_uuid();
	let filter = doc! {
		"$and": [
			{"address": address},
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
