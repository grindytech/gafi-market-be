use actix_web::web::Query;

use super::service;
use crate::{common::QueryCollection, modules::collection::dto::QueryFindCollections};
#[tokio::test]
async fn test_find_collections() {
	let (mut db_process, db) = shared::tests::utils::get_test_db(60000).await;

	let params = QueryCollection {
		search: "".to_string(),
		size: 10,
		order_by: "created_at".to_string(),
		desc: true,
		page: 1,
		query: QueryFindCollections {
			name: None,
			collection_id: None,
			owner: None,
			game_id: None,
		},
	};
	let result = service::find_collections(params, db).await;
	println!("COllection {:?}", result);
	let _ = db_process.kill();
}
