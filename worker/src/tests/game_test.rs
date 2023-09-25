use shared::tests;

use crate::services::{self, game_service};

#[tokio::test]
pub async fn game_add_and_remove_collection() {
	let (mut db_process, db) = tests::utils::get_test_db(60000).await;
	let (_who, public_key) = tests::utils::mock_account_id32();
	
  //create game 0, add collection 0, add collection 1
  game_service::upsert_game_without_metadata("0", &public_key, &db).await.unwrap();
	services::nft_collection::create_collection_without_metadata(&db, "0", &public_key, None)
		.await
		.unwrap();
	services::nft_collection::create_collection_without_metadata(&db, "1", &public_key, None)
		.await
		.unwrap();
	game_service::add_collection("0", "0", &db).await.unwrap();
	game_service::add_collection("0", "1", &db).await.unwrap();

	let game = game_service::get_game_by_id("0", &db).await.unwrap().unwrap();
	let collection0 =
		services::nft_collection::get_collection_by_id(&db, "0").await.unwrap().unwrap();
	let collection1 =
		services::nft_collection::get_collection_by_id(&db, "1").await.unwrap().unwrap();
	assert_eq!(
		game.collections,
		Some(vec!["0".to_string(), "1".to_string()])
	);
	assert_eq!(collection0.games, Some(vec!["0".to_string()]));
	assert_eq!(collection1.games, Some(vec!["0".to_string()]));

  //remove collection 0 from game 0
	game_service::remove_collection("0", "0", &db).await.unwrap();
	let game = game_service::get_game_by_id("0", &db).await.unwrap().unwrap();
	let collection0 =
		services::nft_collection::get_collection_by_id(&db, "0").await.unwrap().unwrap();
	assert_eq!(game.collections, Some(vec!["1".to_string()]));
	assert_eq!(collection0.games, Some(vec![]));

	let _ = db_process.kill();
}
