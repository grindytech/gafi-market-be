use shared::{tests, Pool};

use crate::services::pool_service;

const MOCK_POOL: &str = r#"
{
  "pool_id": "8",
  "owner": "ec84321d9751c066fb923035073a73d467d44642c457915e7496c52f45db1f65",
  "type_pool": "Dynamic",
  "mint_type": "Public",
  "admin": "ec84321d9751c066fb923035073a73d467d44642c457915e7496c52f45db1f65",
  "minting_fee": {
    "$numberDecimal": "20.000000000000000000"
  },
  "begin_at": 218076,
  "end_at": 419676,
  "owner_deposit": "3000000000000000000",
  "updated_at": 1695264049557,
  "created_at": 1695264049557,
  "loot_table": [
    {
      "nft": {
        "collection": "0",
        "item": "0"
      },
      "weight": 20
    }
  ]
}
"#;

#[tokio::test]
pub async fn upsert_pool() {
	let (mut db_process, db) = tests::utils::get_test_db(10000).await;
	let mut pool: Pool = serde_json::from_str(MOCK_POOL).unwrap();
	pool_service::upsert_pool(pool.clone(), &db)
		.await
		.expect("upsert_pool should work");
	let pool_in_db = pool_service::get_pool_by_pool_id(&pool.pool_id, &db)
		.await
		.expect("get pool in db should work");

	pool.id = pool_in_db.clone().unwrap().id;
	assert_eq!(Some(pool), pool_in_db);
	let _ = db_process.kill();
}
