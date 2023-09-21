use mongodb::{bson::doc, bson::Document, options::UpdateOptions, results::UpdateResult, Database};
use shared::{BaseDocument, Pool};

pub async fn upsert_pool(pool: Pool, db: &Database) -> Result<UpdateResult, mongodb::error::Error> {
	let pool_db = db.collection::<Pool>(Pool::name().as_str());
	let option = UpdateOptions::builder().upsert(true).build();
	let query = doc! {"pool_id": pool.pool_id.clone()};

	let pool_doc: Document = pool.into();
	let upsert = doc! {
		"$set": pool_doc
	};

	let rs = pool_db.update_one(query, upsert, option).await?;
	Ok(rs)
}

pub async fn get_pool_by_pool_id(
	pool_id: &str,
	db: &Database,
) -> Result<Option<Pool>, mongodb::error::Error> {
	let pool_db = db.collection::<Pool>(Pool::name().as_str());
	let filter = doc! {"pool_id": pool_id};
	let pool = pool_db.find_one(filter, None).await;
  pool
}
