use chrono::Utc;

use crate::modules::account::{
	dto::{AccountDTO, SocialInfoDto},
	service,
};

#[actix_web::test]
async fn test() {
	let db = shared::tests::utils::get_database().await;
	let address = "test".to_string();
	let id = service::create_account(
		AccountDTO {
			address: address.clone(),
			balance: 0.to_string(),
			is_verified: false,
			name: "test".to_string(),
			bio: "test".to_string(),
			logo_url: None,
			banner_url: None,
			updated_at: Utc::now().timestamp_millis(),
			created_at: Utc::now().timestamp_millis(),
			social: SocialInfoDto {
				discord: None,
				facebook: None,
				medium: None,
				twitter: None,
				web: None,
			},
			favorites: None,
		},
		db.clone(),
	)
	.await
	.ok()
	.unwrap();
	let account = service::get_account(&address.clone(), db.clone()).await.ok().unwrap().unwrap();
	let is_valid = account.address == address;
	service::delete_account_by_address(&address, db).await.expect("Delete error");
	assert_eq!(is_valid, true);
}
