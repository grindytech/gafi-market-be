use crate::services;
use shared::{tests, RequestMint, NFT};

const NFT_MOCK: &str = r#"{
  "_id": {
    "$oid": "650c00ddb36eaeff2fcabcb6"
  },
  "token_id": "0",
  "collection_id": "0",
  "is_burn": null,
  "name": null,
  "description": null,
  "status": null,
  "external_url": null,
  "weight": null,
  "img_url": null,
  "visitor_count": null,
  "favorite_count": null,
  "properties": null,
  "created_at": {
    "$date": {
      "$numberLong": "1695285469739"
    }
  },
  "updated_at": {
    "$date": {
      "$numberLong": "1695285469739"
    }
  },
  "supply": null,
  "created_by": "ec84321d9751c066fb923035073a73d467d44642c457915e7496c52f45db1f65",
  "metadata": null
}"#;

#[tokio::test]
async fn upsert_nft_without_metadata() {
	let (mut db_process, db) = tests::utils::get_test_db(10000).await;
	let mut nft_mock: NFT = serde_json::from_str(NFT_MOCK).unwrap();
	services::nft_service::upsert_nft_without_metadata(
		&nft_mock.collection_id,
		&nft_mock.token_id,
		&nft_mock.created_by,
		nft_mock.supply,
		&db,
	)
	.await
	.unwrap();
	let nft = services::nft_service::get_nft_by_token_id("0", "0", &db)
		.await
		.unwrap()
		.unwrap();

	nft_mock.id = nft.id;
	nft_mock.created_at = nft.created_at;
	nft_mock.updated_at = nft.updated_at;

	assert_eq!(nft, nft_mock);
	let _ = db_process.kill();
}

#[tokio::test]
pub async fn upsert_request_mint() {
	let (mut db_process, db) = tests::utils::get_test_db(10000).await;
	let (_who, public_key) = tests::utils::mock_account_id32();
	let rq_mint = RequestMint {
		block: 0,
		event_index: 0,
		execute_block: 0,
		extrinsic_index: 0,
		pool: "0".to_string(),
		target: public_key.clone(),
		who: public_key.clone(),
	};

	services::nft_service::upsert_request_mint(rq_mint.clone(), &db).await.unwrap();
	let rq = services::nft_service::get_rq_mint(0, 0, &db).await.unwrap().unwrap();

	assert_eq!(rq_mint, rq);
	let _ = db_process.kill();
}

#[tokio::test]
pub async fn nft_metadata_set() {
	let metadata = r#"{
      "title": "hero",
      "image": "/hero.png",
      "other": "other data"
    }"#;
	let (mut db_process, db) = tests::utils::get_test_db(10000).await;
	let nft_mock: NFT = serde_json::from_str(NFT_MOCK).unwrap();
	services::nft_service::upsert_nft_without_metadata(
		&nft_mock.collection_id,
		&nft_mock.token_id,
		&nft_mock.created_by,
		nft_mock.supply,
		&db,
	)
	.await
	.unwrap();

	services::nft_service::nft_metadata_set(
		&metadata,
		&nft_mock.collection_id,
		&nft_mock.token_id,
		&db,
	)
	.await
	.unwrap();

	let nft = services::nft_service::get_nft_by_token_id("0", "0", &db)
		.await
		.unwrap()
		.unwrap();

	let attributes = nft.attributes.unwrap();
	assert_eq!(nft.metadata, Some(metadata.to_string()));
	assert_eq!(attributes.get(0).unwrap().key, "title");
	assert_eq!(attributes.get(0).unwrap().value, "\"hero\"");
	assert_eq!(attributes.get(1).unwrap().key, "image");
	assert_eq!(attributes.get(1).unwrap().value, "\"/hero.png\"");
	assert_eq!(attributes.get(2).unwrap().key, "other");
	assert_eq!(attributes.get(2).unwrap().value, "\"other data\"");

	//meta data not in json format
	let metadata = r#""other": "other data""#;
	services::nft_service::nft_metadata_set(
		&metadata,
		&nft_mock.collection_id,
		&nft_mock.token_id,
		&db,
	)
	.await
	.unwrap();
	let nft = services::nft_service::get_nft_by_token_id("0", "0", &db)
		.await
		.unwrap()
		.unwrap();
	assert_eq!(nft.metadata, Some(metadata.to_string()));

	// clear metadata
	services::nft_service::clear_metadata(&nft.collection_id, &nft.token_id, &db)
		.await
		.unwrap();
	let nft = services::nft_service::get_nft_by_token_id("0", "0", &db)
		.await
		.unwrap()
		.unwrap();
	assert_eq!(None, nft.metadata);
	assert_eq!(None, nft.attributes);

	let _ = db_process.kill();
}
