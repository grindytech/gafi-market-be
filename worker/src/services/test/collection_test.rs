use crate::{gafi::game::events::CollectionCreated, services};
use shared::tests;

#[tokio::test]
async fn on_collection_created_handle() {
	let (mut db_process, db) = tests::utils::get_test_db(60000).await;
	let (_who, public_key) = tests::utils::mock_account_id32();
	let rs = services::nft_collection::upsert_without_metadata("0", &public_key, &db).await;
	match rs {
		Ok(_) => {
			let nft_collection = services::nft_collection::get_collection_by_id(&db, "0").await;
			let nft_collection = nft_collection.unwrap().unwrap();
			assert!(nft_collection.collection_id == "0" && nft_collection.owner == public_key)
		},
		Err(_) => assert!(false, "on_collection_created_handle must be Ok"),
	}
	let _ = db_process.kill();
}

#[tokio::test]
async fn on_collection_metadata_set_handle() {
	let (mut db_process, db) = tests::utils::get_test_db(60000).await;
	let metadata = r#"
		{
			"title": "chess",
			"image": "/chess.svg",
			"external_url": "https://chess.com",
			"other": "other data"
		}
	"#;
	let collection = 0;
	let (_who, public_key) = tests::utils::mock_account_id32();
	let _insert = services::nft_collection::create_collection_without_metadata(
		&db,
		&collection.to_string(),
		&public_key,
		None,
	)
	.await
	.unwrap();

	let _rs =
		services::nft_collection::update_collection_metadata(metadata.to_string(), collection, &db)
			.await
			.unwrap();

	let nft_collection =
		services::nft_collection::get_collection_by_id(&db, &collection.to_string()).await;
	let nft_collection = nft_collection.unwrap().unwrap();
	assert!(metadata == nft_collection.metadata.unwrap());
	let _ = db_process.kill();
}
