use std::collections::HashMap;

use mongodb::{
	bson::{doc, DateTime},
	options::UpdateOptions,
};
use serde::Deserialize;
pub use shared::types::Result;
use shared::{BaseDocument, NFTOwner, NFT};

use crate::{
	gafi, services,
	workers::{HandleParams, Task},
};

#[derive(Deserialize, Debug)]
struct NftMetadata {
	title: String,
	image: String,
}

async fn on_metadata_set(params: HandleParams<'_>) -> Result<()> {
	let event_parse = params.ev.as_event::<gafi::nfts::events::ItemMetadataSet>()?;
	if let Some(ev) = event_parse {
		let data = String::from_utf8(ev.data.0).ok();
		if let Some(metadata) = data {
			let object: std::result::Result<NftMetadata, _> = serde_json::from_str(&metadata);
			match object {
				Ok(data) => {
					let nft_db = params.db.collection::<NFT>(NFT::name().as_str());
					let query = doc! {"token_id": ev.item.to_string(),"collection_id": ev.collection.to_string()};
					let update = doc! {"$set": {
						"img_url": data.image,
						"name": data.title,
						"updated_at": DateTime::now()
					}};
					nft_db.update_one(query, update, None).await?;
					log::info!(
						"ItemMetadataSet collection {}, token id {}, data: {}",
						ev.collection,
						ev.item,
						metadata
					);
				},
				Err(err) => {
					log::warn!("{:?}", err)
				},
			}
		}
	}
	Ok(())
}
//ItemAdded
async fn on_item_added(params: HandleParams<'_>) -> Result<()> {
	let event_parse = params.ev.as_event::<gafi::game::events::ItemAdded>()?;
	if let Some(ev) = event_parse {
		let nft_db = params.db.collection::<NFT>(NFT::name().as_str());
		let query =
			doc! {"token_id": ev.item.to_string(),"collection_id": ev.collection.to_string()};

		let update = doc! {"$set": {
			"supply": ev.amount,
			"updated_at": DateTime::now()
		}};

		nft_db.update_one(query, update, None).await?;
		log::info!("Nft item added {:?}", ev);
	}

	Ok(())
}
//ItemCreated
async fn on_item_created(params: HandleParams<'_>) -> Result<()> {
	let event_parse = params.ev.as_event::<gafi::game::events::ItemCreated>()?;
	if let Some(ev) = event_parse {
		let nft_db = params.db.collection::<NFT>(NFT::name().as_str());
		let options = UpdateOptions::builder().upsert(true).build();
		let query =
			doc! {"token_id": ev.item.to_string(),"collection_id": ev.collection.to_string()};

		let upsert = doc! {"$set": {
			"token_id": ev.item.to_string(),
			"collection_id": ev.collection.to_string(),
			"created_by": hex::encode(ev.who.0),
			"supply": ev.maybe_supply,
			"created_at": DateTime::now(),
			"updated_at": DateTime::now()
		}};
		nft_db.update_one(query, upsert, options).await?;
		log::info!("Nft item created {:?}", ev);
	}
	Ok(())
}

async fn on_mint_nft(params: HandleParams<'_>) -> Result<()> {
	let event_parse = params.ev.as_event::<gafi::game::events::Minted>()?;
	if let Some(ev) = event_parse {
		let mut need_refetch_amount = HashMap::<String, bool>::new();

		for item in ev.nfts {
			need_refetch_amount.insert(
				format!("{}:{}", item.collection, item.item).to_string(),
				true,
			);
		}
		//get balances & update
		for key in need_refetch_amount.keys() {
			let mut arr_str = key.split(":");

			let collection_id = arr_str.next().unwrap();
			let token_id = arr_str.next().unwrap();

			services::nft::refresh_balance(
				ev.target.clone(),
				collection_id.to_string(),
				token_id.to_string(),
				params.db,
				params.api,
			)
			.await?;
		}
	};
	Ok(())
}

pub fn tasks() -> Vec<Task> {
	vec![
		Task::new("Game:Minted", move |params| Box::pin(on_mint_nft(params))),
		Task::new("Game:ItemCreated", move |params| {
			Box::pin(on_item_created(params))
		}),
		Task::new("Game:ItemAdded", move |params| {
			Box::pin(on_item_added(params))
		}),
		Task::new("Nfts:ItemMetadataSet", move |params| {
			Box::pin(on_metadata_set(params))
		}),
	]
}

//TODO
//Nfts:transfer
//Nfts:burn
//Nfts:issued
