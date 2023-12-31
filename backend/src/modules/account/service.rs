use crate::common::{ErrorResponse, QueryPage};
use actix_web::{http, Result};
use mongodb::{
	bson::doc, error::Error, options::FindOneAndUpdateOptions, results::DeleteResult, Collection,
	Database,
};
use shared::{
	models::{self, account::Account},
	BaseDocument,
};

use super::dto::{AccountDTO, QueryFindAccount};

pub async fn find_account_by_adress(
	address: &String,
	db: Database,
) -> Result<Option<AccountDTO>, mongodb::error::Error> {
	let col: Collection<Account> = db.collection(models::Account::name().as_str());
	let filter = doc! {"address": address};
	if let Ok(Some(account_detail)) = col.find_one(filter, None).await {
		Ok(Some(account_detail.into()))
	} else {
		Ok(None)
	}
}

pub async fn get_account(
	address: &String,
	db: Database,
) -> Result<Option<Account>, mongodb::error::Error> {
	let col: Collection<Account> = db.collection(models::Account::name().as_str());
	let filter = doc! {"address": address};
	col.find_one(filter, None).await
}

pub async fn create_account(
	account: AccountDTO,
	db: Database,
) -> Result<String, mongodb::error::Error> {
	let col: Collection<Account> = db.collection(models::Account::name().as_str());
	let entity: Account = Account {
		address: account.address,
		balance: account.balance,
		banner: account.banner,
		bio: account.bio,
		logo: account.logo,
		name: account.name,
		id: None,
		is_verified: None,
		social: account.social.into(),
		updated_at: account.updated_at,
		created_at: account.created_at,
		favorites: None,
		nonce: None,
		refresh_token: None,
	};
	let rs = col.insert_one(entity.clone(), None).await;
	match rs {
		Ok(r) => Ok(r.inserted_id.to_string()),
		Err(e) => Err(e),
	}
}
pub async fn delete_account_by_address(
	address: &str,
	db: Database,
) -> Result<DeleteResult, mongodb::error::Error> {
	let collection: Collection<Account> = db.collection(models::Account::name().as_str());
	let filter = doc! {"address": address};
	collection.delete_one(filter, None).await
}

pub async fn update_favorites_account(
	params: QueryPage<QueryFindAccount>,
	db: Database,
) -> Result<Option<AccountDTO>, Error> {
	let collection: Collection<Account> = db.collection(models::Account::name().as_str());
	let filter = doc! {"address":params.query.address};

	let update = doc! {
		"$set":{"favorites":&params.query.favorites}
	};
	let options = FindOneAndUpdateOptions::builder()
		.return_document(mongodb::options::ReturnDocument::After)
		.build();
	if let Ok(Some(result)) = collection.find_one_and_update(filter, update, options).await {
		Ok(Some(result.into()))
	} else {
		Ok(None)
	}
}
