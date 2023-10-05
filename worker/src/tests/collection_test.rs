use crate::services;
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
async fn collection_metadata_set() {
	let (mut db_process, db) = tests::utils::get_test_db(60000).await;
	let metadata = r#"
		{
			"title": "chess",
			"image": "/chess.svg",
			"external_url": "https://chess.com"
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
		services::nft_collection::get_collection_by_id(&db, &collection.to_string())
			.await
			.unwrap()
			.unwrap();

	assert_eq!(nft_collection.banner_url, Some("".to_string()));
	assert_eq!(nft_collection.name, Some("chess".to_string()));
	assert_eq!(
		nft_collection.external_url,
		Some("https://chess.com".to_string())
	);
	assert_eq!(nft_collection.logo_url, Some("/chess.svg".to_string()));

	//meta data not in json format
	let metadata = r#""other": "other data""#;
	let _rs =
		services::nft_collection::update_collection_metadata(metadata.to_string(), collection, &db)
			.await
			.unwrap();

	let nft_collection =
		services::nft_collection::get_collection_by_id(&db, &collection.to_string())
			.await
			.unwrap()
			.unwrap();

	assert_eq!(nft_collection.banner_url, None);
	assert_eq!(nft_collection.name, None);
	assert_eq!(nft_collection.external_url, None);
	assert_eq!(nft_collection.logo_url, None);

	let _ = db_process.kill();
}

#[tokio::test]
async fn collection_metadata_cleared() {
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

	let _clear = services::nft_collection::clear_metadata(&collection.to_string(), &db)
		.await
		.unwrap();
	let nft_collection =
		services::nft_collection::get_collection_by_id(&db, &collection.to_string())
			.await
			.unwrap()
			.unwrap();

	assert_eq!(nft_collection.banner_url, None);
	assert_eq!(nft_collection.name, None);
	assert_eq!(nft_collection.external_url, None);
	assert_eq!(nft_collection.logo_url, None);

	let _ = db_process.kill();
}
