use crate::models::{self, account::Account};
use actix_web::{http, Result};
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    Client, Collection, Database,
};

use super::dto::AccountDTO;

pub async fn get_account(address: &String, db: Database) -> Result<Option<AccountDTO>, Error> {
    let col: Collection<Account> = db.collection(models::account::NAME);

    let filter = doc! {"address": address};

    if let Ok(Some(account)) = col.find_one(filter, None).await {
        let account_dto = AccountDTO {
            name: account.name,
            bio: account.bio,
            logo_url: account.logo_url,
            address: account.address,
            balance: account.balance,
            is_verified: account.is_verified,
            banner_url: account.banner_url,
            // Add other fields if needed
        };
        Ok(Some(account_dto))
    } else {
        Ok(None)
    }
}
