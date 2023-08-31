use mongodb::{bson::doc, Collection, Database};
use shared::{models, Account, BaseDocument};

use crate::common::utils::generate_random_six_digit_number;

use super::dto::QueryAuth;
/**
 *  1. FE initialize Sign in  => 2. Backend generate Nonce => 3. Store Nonce in database
 * 2. FE Fetch the nonce => Sign the nonce => Backend get the Signature => Detech Signature => If true return access token and change nonce
 */
pub async fn update_nonce(
	address: &String,
	nonce: u32,
	db: Database,
) -> Result<Option<Account>, mongodb::error::Error> {
	let col: Collection<Account> = db.collection(models::account::Account::name().as_str());

	let filter = doc! {"address":address};
	let update = doc! {
		"$set":{"nonce":nonce}
	};
	if let Ok(Some(account)) = col.find_one_and_update(filter, update, None).await {
		Ok(Some(account))
	} else {
		Ok(None)
	}
}

pub async fn get_jwt_token(
	params: QueryAuth,
	db: Database,
) -> Result<Option<Account>, mongodb::error::Error> {
	let address = params.address;
	let signature = params.signature; // nonce

	let collection: Collection<Account> = db.collection(models::Account::name().as_str());
	let nonce = generate_random_six_digit_number();
	let filter = doc! {
		"$and": [
			{"address": address},
			{"nonce": signature},
		],

	};
	let update = doc! {
		"$set":{"nonce":nonce}
	};

	if let Ok(Some(account_detail)) = collection.find_one_and_update(filter, update, None).await {
		Ok(Some(account_detail))
	} else {
		Ok(None)
	}
}
