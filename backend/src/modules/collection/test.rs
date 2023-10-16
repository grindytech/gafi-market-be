use actix_web::web::Query;
use mongodb::bson::{doc, Document};
use shared::{models, BaseDocument, HistoryTx};

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
#[tokio::test]
async fn test_collection_analysis() {
	let collection_id = "1".to_string();
	let (mut db_process, db) = shared::tests::utils::get_test_db(60000).await;
	let col: mongodb::Collection<HistoryTx> =
		db.collection(models::history_tx::HistoryTx::name().as_str());
	let pipeline: Vec<Document> = vec![doc! {
		"$group": {
			"collection_id": collection_id,
			"minPrice": { "$min": "$price" },
			"maxPrice": { "$max": "$price" },

		}
	}];
	let cursor = col.aggregate(pipeline, None).await.unwrap();
	println!("Cursor {:?}", cursor);
	let _ = db_process.kill();
}
