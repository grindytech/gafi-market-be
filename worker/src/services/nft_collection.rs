use mongodb::{
	bson::{doc, Bson, DateTime},
	error,
	options::{InsertOneOptions, UpdateOptions},
	results::UpdateResult,
	Database,
};
use serde_json::Value;
use shared::{utils::serde_json_to_doc, BaseDocument, NFTCollection};

pub async fn get_collection_by_id(
	db: &Database,
	collection_id: &str,
) -> Result<Option<NFTCollection>, error::Error> {
	let collection_db = db.collection::<NFTCollection>(&NFTCollection::name());
	let nft_collection = collection_db
		.find_one(
			doc! {
			  "collection_id": collection_id,
			},
			None,
		)
		.await?;
	Ok(nft_collection)
}

pub async fn create_collection_without_metadata(
	db: &Database,
	collection_id: &str,
	who: &str,
	option: Option<InsertOneOptions>,
) -> Result<mongodb::results::InsertOneResult, error::Error> {
	let collection_db = db.collection::<NFTCollection>(&NFTCollection::name());
	let nft_collection = collection_db
		.insert_one(
			NFTCollection {
				collection_id: collection_id.to_string(),
				owner: who.to_string(),
				created_at: DateTime::now(),
				updated_at: Some(DateTime::now()),
				external_url: None,
				games: None,
				id: None,
				is_verified: None,
				logo_url: None,
				name: None,
				slug: None,
				banner_url: None,
				category: None,
				metadata: None,
				attributes: None,
			},
			option,
		)
		.await?;
	Ok(nft_collection)
}

pub async fn upsert_without_metadata(
	collection_id: &str,
	who: &str,
	db: &Database,
) -> Result<UpdateResult, mongodb::error::Error> {
	let collection_db: mongodb::Collection<NFTCollection> =
		db.collection::<NFTCollection>(NFTCollection::name().as_str());
	let option = UpdateOptions::builder().upsert(true).build();
	let query = doc! {"collection_id": collection_id.to_string()};
	let new_collection = doc! {
			"$set": {
		  "collection_id": collection_id.to_string(),
		  "owner": who,
			"created_at": DateTime::now(),
			"updated_at": DateTime::now(),
		}
	};
	let rs = collection_db.update_one(query, new_collection, option).await?;
	log::info!("NFT Collection created {} {}", collection_id, who);
	Ok(rs)
}

pub async fn update_collection_metadata(
	metadata: String,
	collection: u32,
	db: &Database,
) -> Result<UpdateResult, mongodb::error::Error> {
	let parsed: Result<Value, serde_json::Error> = serde_json::from_str(&metadata);
	let update;
	match parsed {
		Ok(data) => {
			let parsed_obj = serde_json_to_doc(data);
			match parsed_obj {
				Ok((doc, obj)) => {
					let empty_val = Value::String("".to_string());
					let image = obj.get("image").unwrap_or(&empty_val).as_str().unwrap_or("");
					let title = obj.get("title").unwrap_or(&empty_val).as_str().unwrap_or("");
					let external_url =
						obj.get("external_url").unwrap_or(&empty_val).as_str().unwrap_or("");
					update = doc! {
							"$set": {
							"logo_url": image.to_string(),
							"name": title.to_string(),
							"external_url": external_url.to_string(),
							"updated_at": DateTime::now(),
							"metadata": metadata.clone(),
							"attributes": doc,
						}
					};
				},
				Err(_) => {
					update = doc! {
							"$set": {
							"logo_url": Bson::Null,
							"name": Bson::Null,
							"external_url": Bson::Null,
							"updated_at": DateTime::now(),
							"metadata": metadata.clone(),
							"attributes": Bson::Null,
						}
					};
				},
			}
		},
		Err(_) => {
			update = doc! {"$set": {
				"updated_at": DateTime::now(),
				"metadata": metadata.clone(),
				"logo_url": Bson::Null,
				"name": Bson::Null,
				"external_url": Bson::Null,
				"attributes": Bson::Null,
			}};
		},
	}
	let collection_db: mongodb::Collection<NFTCollection> =
		db.collection::<NFTCollection>(NFTCollection::name().as_str());
	let option = UpdateOptions::builder().upsert(true).build();
	let query = doc! {"collection_id": collection.to_string()};
	let rs = collection_db.update_one(query, update, option).await?;
	Ok(rs)
}

pub async fn clear_metadata(
	collection_id: &str,
	db: &Database,
) -> Result<UpdateResult, mongodb::error::Error> {
	let collection_db: mongodb::Collection<NFTCollection> =
		db.collection::<NFTCollection>(NFTCollection::name().as_str());

	let query = doc! {"collection_id": collection_id.to_string()};
	let new_collection = doc! {
			"$set": {
				"metadata": Bson::Null,
				"logo_url": Bson::Null,
				"name": Bson::Null,
				"external_url": Bson::Null,
				"attributes": Bson::Null,
				"updated_at": DateTime::now(),
		}
	};
	let rs = collection_db.update_one(query, new_collection, None).await?;
	Ok(rs)
}
