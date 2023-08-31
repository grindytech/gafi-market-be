use std::collections::HashMap;

use mongodb::{
	bson::{doc, DateTime, Decimal128, Document},
	options::UpdateOptions,
};
use serde::Deserialize;
pub use shared::types::Result;
use shared::{
	constant::{
		EVENT_ITEM_ADDED, EVENT_ITEM_CREATED, EVENT_ITEM_METADATA_SET, EVENT_MINTED,
		EVENT_REQUEST_MINT, EVENT_TRANSFERRED,
	},
	BaseDocument, HistoryTx, RequestMint, NFT,
};

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
						"updated_at": DateTime::now(),
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
		services::nft::refresh_supply(ev.collection, ev.item, params.db, params.api).await?;
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
			"updated_at": DateTime::now(),
		}};
		nft_db.update_one(query, upsert, options).await?;
		log::info!("Nft item created {:?}", ev);
	}
	Ok(())
}
async fn on_mint_nft(params: HandleParams<'_>) -> Result<()> {
	let event_parse = params.ev.as_event::<gafi::game::events::Minted>()?;
	if let Some(ev) = event_parse {
		let mut need_refetch_amount = HashMap::<String, u32>::new();
		let mut nfts = vec![];

		for item in ev.nfts {
			let key = format!("{}:{}", item.collection, item.item).to_string();
			let amount = need_refetch_amount.get(&key).unwrap_or(&0);
			need_refetch_amount.insert(key, amount + 1);
		}
		//get balances & update
		for key in need_refetch_amount.keys() {
			let mut arr_str = key.split(":");
			let collection_id = arr_str.next().expect("get collection_id fail");
			let token_id = arr_str.next().expect("get token_id fail");
			let amount = need_refetch_amount.get(key).expect("fail to get need_refetch_amount");
			nfts.push(shared::history_tx::Nft {
				amount: *amount,
				collection: collection_id.parse()?,
				item: token_id.parse()?,
			});

			services::nft::refresh_balance(
				ev.target.clone(),
				collection_id.to_string(),
				token_id.to_string(),
				params.db,
				params.api,
			)
			.await?;
		}
		let config = shared::config::Config::init();
		let amount = ev.amount;
		let price = ev.price;
		let price_str = shared::utils::string_decimal_to_number(
			&price.to_string(),
			config.chain_decimal as i32,
		);
		let price_decimal: Decimal128 = price_str.parse()?;
		let total_value = price_str.parse::<f64>()? * f64::from(amount);
		let value_in_decimal: Decimal128 = total_value.to_string().parse()?;

		services::history::upsert(
			HistoryTx {
				event: "Game:Minted".to_string(),
				event_index: params.ev.index(),
				extrinsic_index: params.extrinsic_index.unwrap(),
				block_height: params.block.height,
				from: hex::encode(ev.who.0),
				to: Some(hex::encode(ev.target.0)),
				value: Some(value_in_decimal),
				id: None,
				tx_hash: None,
				pool: Some(ev.pool.to_string()),
				nfts: Some(nfts),
				price: Some(price_decimal),
				amount: Some(amount),
				trade_id: None,
				source: None,
				trade_type: None,
			},
			params.db,
		)
		.await?;
	};

	Ok(())
}

async fn on_item_transfer(params: HandleParams<'_>) -> Result<()> {
	let event_parse = params.ev.as_event::<gafi::game::events::Transferred>()?;
	if let Some(ev) = event_parse {
		let nft = shared::history_tx::Nft {
			amount: ev.amount.into(),
			collection: ev.collection,
			item: ev.item,
		};
		services::history::upsert(
			HistoryTx {
				event: "Game:Transferred".to_string(),
				event_index: params.ev.index(),
				extrinsic_index: params.extrinsic_index.unwrap(),
				block_height: params.block.height,
				from: hex::encode(ev.from.0),
				to: Some(hex::encode(ev.dest.0)),
				value: None,
				id: None,
				tx_hash: None,
				pool: None,
				nfts: Some(vec![nft]),
				amount: Some(1),
				price: None,
				trade_id: None,
				source: None,
				trade_type: None,
			},
			params.db,
		)
		.await?;

		services::nft::refresh_balance(
			ev.from.clone(),
			ev.collection.to_string(),
			ev.item.to_string(),
			params.db,
			params.api,
		)
		.await?;

		services::nft::refresh_balance(
			ev.dest.clone(),
			ev.collection.to_string(),
			ev.item.to_string(),
			params.db,
			params.api,
		)
		.await?;
	}
	Ok(())
}

async fn on_request_mint(params: HandleParams<'_>) -> Result<()> {
	let event_parse = params.ev.as_event::<gafi::game::events::RequestMint>()?;
	if let Some(ev) = event_parse {
		let request_mint_db = params.db.collection::<RequestMint>(RequestMint::name().as_str());
		let rq: Document = RequestMint {
			block: params.block.height,
			event_index: params.ev.index(),
			execute_block: ev.block_number,
			extrinsic_index: params.extrinsic_index.unwrap(),
			pool: ev.pool.to_string(),
			target: hex::encode(ev.target.0),
			who: hex::encode(ev.who.0),
		}
		.into();
		let query = doc! {
			"block": params.block.height,
			"event_index": params.ev.index(),
			"extrinsic_index": params.extrinsic_index.unwrap(),
		};
		let update = doc! {
			"$set": rq,
		};
		let options = UpdateOptions::builder().upsert(true).build();
		request_mint_db.update_one(query, update, options).await?;
	}
	Ok(())
}

pub fn tasks() -> Vec<Task> {
	vec![
		Task::new(EVENT_MINTED, move |params| Box::pin(on_mint_nft(params))),
		Task::new(EVENT_ITEM_CREATED, move |params| {
			Box::pin(on_item_created(params))
		}),
		Task::new(EVENT_ITEM_ADDED, move |params| {
			Box::pin(on_item_added(params))
		}),
		Task::new(EVENT_ITEM_METADATA_SET, move |params| {
			Box::pin(on_metadata_set(params))
		}),
		Task::new(EVENT_TRANSFERRED, move |params| {
			Box::pin(on_item_transfer(params))
		}),
		Task::new(EVENT_REQUEST_MINT, move |params| {
			Box::pin(on_request_mint(params))
		}),
	]
}
