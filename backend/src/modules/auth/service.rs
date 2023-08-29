use mongodb::{bson::doc, Collection, Database};
use shared::{models, Account, BaseDocument};

use super::dto::QueryAuth;

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
	let signature = params.signature;

	let collection: Collection<Account> = db.collection(models::Account::name().as_str());
	let filter = doc! {
		"$and": [
			{"address": address},
			{"nonce": signature},
		]
	};

	if let Ok(Some(account_detail)) = collection.find_one(filter, None).await {
		Ok(Some(account_detail))
	} else {
		Ok(None)
	}
}
