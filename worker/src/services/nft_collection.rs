use mongodb::{
	bson::{doc, Bson, DateTime},
	error,
	options::{InsertOneOptions, UpdateOptions},
	Database,
};
use serde::Deserialize;
use shared::{BaseDocument, NFTCollection};

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
) -> shared::Result<()> {
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
	collection_db.update_one(query, new_collection, option).await?;
	log::info!("NFT Collection created {} {}", collection_id, who);
	Ok(())
}

#[derive(Deserialize, Debug)]
struct CollectionMetadata {
	title: Option<String>,
	image: Option<String>,
	external_url: Option<String>,
}
pub async fn update_collection_metadata(
	metadata: String,
	collection: u32,
	db: &Database,
) -> shared::Result<()> {
	let object = serde_json::from_str::<CollectionMetadata>(&metadata);
	let update;
	match object {
		Ok(data) => {
			update = doc! {
					"$set": {
					"logo_url": data.image,
					"name": data.title,
					"updated_at": DateTime::now(),
					"external_url": data.external_url,
					"metadata": metadata.clone(),
				}
			};
		},
		Err(_) => {
			update = doc! {"$set": {
				"updated_at": DateTime::now(),
				"metadata": metadata.clone(),
				"logo_url": Bson::Null,
				"name": Bson::Null,
				"external_url": Bson::Null,
			}};
		},
	}
	let collection_db: mongodb::Collection<NFTCollection> =
		db.collection::<NFTCollection>(NFTCollection::name().as_str());
	let option = UpdateOptions::builder().upsert(true).build();
	let query = doc! {"collection_id": collection.to_string()};
	collection_db.update_one(query, update, option).await?;
	Ok(())
}
