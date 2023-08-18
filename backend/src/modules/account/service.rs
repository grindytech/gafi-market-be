use crate::common::ErrorResponse;
use actix_web::{http, Result};
use mongodb::{bson::doc, results::DeleteResult, Collection, Database};
use shared::models::{self, account::Account};

use super::dto::AccountDTO;

pub async fn find_account_by_adress(
	address: &String,
	db: Database,
) -> Result<Option<AccountDTO>, mongodb::error::Error> {
	let col: Collection<Account> = db.collection(models::account::NAME);
	let filter = doc! {"address": address};
	if let Ok(Some(account_detail)) = col.find_one(filter, None).await {
		Ok(Some(account_detail.into()))
	} else {
		Ok(None)
	}
}
pub async fn find_account_by_name(
	name: &String,
	db: Database,
) -> Result<Option<AccountDTO>, mongodb::error::Error> {
	let col: Collection<Account> = db.collection(models::account::NAME);
	let filter = doc! {"name": name};
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
	let col: Collection<Account> = db.collection(models::account::NAME);
	let filter = doc! {"address": address};
	col.find_one(filter, None).await
}

pub async fn create_account(account: AccountDTO, db: Database) -> Result<String, ErrorResponse> {
	let col: Collection<Account> = db.collection(models::account::NAME);
	let entity: Account = Account {
		address: account.address,
		balance: account.balance,
		banner_url: account.banner_url,
		bio: account.bio,
		logo_url: account.logo_url,
		name: account.name,
		id: None,
		is_verified: false,
		social: account.social.into(),
		update_at: account.update_at,
		create_at: account.create_at,
	};
	let rs = col.insert_one(entity.clone(), None).await;
	match rs {
		Ok(r) => Ok(r.inserted_id.to_string()),
		Err(e) => Err(ErrorResponse {
			message: e.to_string(),
			status: http::StatusCode::INTERNAL_SERVER_ERROR.as_u16().to_string(),
		}),
	}
}
pub async fn delete_account_by_address(
	address: &str,
	db: Database,
) -> Result<DeleteResult, mongodb::error::Error> {
	let collection: Collection<Account> = db.collection(models::account::NAME);
	let filter = doc! {"address": address};
	collection.delete_one(filter, None).await
}
