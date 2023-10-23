use mongodb::{
	bson::{doc, Bson, Document},
	options::UpdateOptions,
	results::UpdateResult,
	Database,
};
use shared::{utils::serde_json_to_doc, BaseDocument, Pool};

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
pub async fn update_metadata(
	metadata: String,
	pool_id: u32,
	db: &Database,
) -> Result<UpdateResult, mongodb::error::Error> {
	let parsed: Result<serde_json::Value, serde_json::Error> = serde_json::from_str(&metadata);
	let update;
	match parsed {
		Ok(data) => {
			let parase_obj = serde_json_to_doc(data);
			match parase_obj {
				Ok((_doc, object)) => {
					let empty_val = serde_json::Value::String("".to_string());
					let title = object.get("title").unwrap_or(&empty_val).as_str().unwrap_or("");
					let description =
						object.get("description").unwrap_or(&empty_val).as_str().unwrap_or("");
					update = doc! {
						"$set":{
							"title":title.to_string(),
							"description":description.to_string()
						}
					};
				},
				Err(_) => {
					update = doc! {
						"$set":{
							"title":Bson::Null,
							"description":Bson::Null,
						}
					};
				},
			}
		},
		Err(_) => {
			update = doc! {
				"$set":{
					"title":Bson::Null,
					"description":Bson::Null,
				}
			};
		},
	}
	let pool_db: mongodb::Collection<Pool> = db.collection::<Pool>(Pool::name().as_str());
	let option = UpdateOptions::builder().upsert(true).build();
	let query = doc! {"pool_id":pool_id.to_string()};
	let rs = pool_db.update_one(query, update, option).await?;
	Ok(rs)
}

pub async fn clear_metadata(
	pool_id: u32,
	db: &Database,
) -> Result<UpdateResult, mongodb::error::Error> {
	let pool_db: mongodb::Collection<Pool> = db.collection::<Pool>(Pool::name().as_str());
	let query = doc! {"pool_id":pool_id.to_string()};
	let update = doc! {
		"$set":{
			"title":Bson::Null,
			"description":Bson::Null,
		}
	};
	let rs: UpdateResult = pool_db.update_one(query, update, None).await?;
	Ok(rs)
}
