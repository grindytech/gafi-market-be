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
				games: None,
				id: None,
				is_verified: None,
				slug: None,
				category: None,
				banner: None,
				external_url: None,
				logo: None,
				cover: None,
				name: None,
				description: None,
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
				Ok((_doc, obj)) => {
					let empty_val = Value::String("".to_string());
					let banner = obj.get("banner").unwrap_or(&empty_val).as_str().unwrap_or("");
					let logo = obj.get("logo").unwrap_or(&empty_val).as_str().unwrap_or("");
					let cover = obj.get("cover").unwrap_or(&empty_val).as_str().unwrap_or("");
					let name = obj.get("name").unwrap_or(&empty_val).as_str().unwrap_or("");
					let description =
						obj.get("description").unwrap_or(&empty_val).as_str().unwrap_or("");
					let external_url =
						obj.get("external_url").unwrap_or(&empty_val).as_str().unwrap_or("");
					update = doc! {
							"$set": {
								"logo": logo.to_string(),
								"banner":banner.to_string(),
								"cover":cover.to_string(),
								"name": name.to_string(),
								"description":description.to_string(),
								"external_url": external_url.to_string(),
								"updated_at": DateTime::now(),
							}
					};
				},
				Err(_) => {
					update = doc! {
					"$set": {
						"logo": Bson::Null,
						"banner": Bson::Null,
						"cover":Bson::Null,
						"name": Bson::Null,
						"description":Bson::Null,
						"external_url": Bson::Null,
						"updated_at": DateTime::now(),
						}
					};
				},
			}
		},
		Err(_) => {
			update = doc! {
				"$set": {
						"logo": Bson::Null,
						"banner": Bson::Null,
						"cover":Bson::Null,
						"name": Bson::Null,
						"description":Bson::Null,
						"external_url": Bson::Null,
						"updated_at": DateTime::now(),
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
				"logo": Bson::Null,
				"banner": Bson::Null,
				"description":Bson::Null,
				"cover":Bson::Null,
				"name": Bson::Null,
				"external_url": Bson::Null,
				"updated_at": DateTime::now(),
		}
	};
	let rs = collection_db.update_one(query, new_collection, None).await?;
	Ok(rs)
}
