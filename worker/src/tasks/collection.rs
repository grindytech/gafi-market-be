use mongodb::{
	bson::{doc, DateTime},
	options::UpdateOptions,
};
pub use shared::types::Result;

use crate::{
	gafi::{self, game::events::CollectionCreated},
	workers::{HandleParams, Task},
};
use serde::Deserialize;

use shared::{
	constant::{EVENT_COLLECTION_CREATED, EVENT_COLLECTION_METADATA_SET},
	BaseDocument, NFTCollection,
};

async fn on_collection_created(params: HandleParams<'_>) -> Result<()> {
	let event_parse = params.ev.as_event::<CollectionCreated>()?;
	if let Some(collection) = event_parse {
		let collection_db: mongodb::Collection<NFTCollection> =
			params.db.collection::<NFTCollection>(NFTCollection::name().as_str());
		let option = UpdateOptions::builder().upsert(true).build();
		let query = doc! {"collection_id": collection.collection.to_string()};
		let new_collection = doc! {
				"$set": {
			  "collection_id": collection.collection.to_string(),
			  "owner": hex::encode(collection.who.0),
				"created_at": DateTime::now(),
				"updated_at": DateTime::now(),
			}
		};
		collection_db.update_one(query, new_collection, option).await?;
		log::info!(
			"NFT Collection created {} {}",
			collection.collection,
			collection.who
		);
	};
	Ok(())
}

#[derive(Deserialize, Debug)]
struct CollectionMetadata {
	title: String,
	image: String,
	external_url: String,
}
//CollectionMetadataSet
async fn on_collection_metadata_set(params: HandleParams<'_>) -> Result<()> {
	let event_parse = params.ev.as_event::<gafi::nfts::events::CollectionMetadataSet>()?;
	if let Some(ev) = event_parse {
		let data = String::from_utf8(ev.data.0).ok();
		if let Some(metadata) = data {
			let object: std::result::Result<CollectionMetadata, _> =
				serde_json::from_str(&metadata);
			match object {
				Ok(data) => {
					let collection_db: mongodb::Collection<NFTCollection> =
						params.db.collection::<NFTCollection>(NFTCollection::name().as_str());
					let option = UpdateOptions::builder().upsert(true).build();
					let query = doc! {"collection_id": ev.collection.to_string()};
					let update = doc! {"$set": {
						"logo_url": data.image,
						"name": data.title,
						"updated_at": DateTime::now(),
						"external_url": data.external_url,
					}};
					collection_db.update_one(query, update, option).await?;
					log::info!(
						"CollectionMetadataSet collection {}, data: {}",
						ev.collection,
						metadata
					);
				},
				Err(err) => {
					log::warn!("{:?}", err)
				},
			}
		}
	};
	Ok(())
}

pub fn tasks() -> Vec<Task> {
	vec![
		Task::new(EVENT_COLLECTION_CREATED, move |params| {
			Box::pin(on_collection_created(params))
		}),
		Task::new(EVENT_COLLECTION_METADATA_SET, move |params| {
			Box::pin(on_collection_metadata_set(params))
		}),
	]
}
