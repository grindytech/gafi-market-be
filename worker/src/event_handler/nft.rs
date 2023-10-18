use std::collections::HashMap;

use mongodb::bson::Decimal128;
pub use shared::types::Result;
use shared::{
	constant::{
		EVENT_ITEM_ADDED, EVENT_ITEM_CREATED, EVENT_ITEM_METADATA_CLEARED, EVENT_ITEM_METADATA_SET,
		EVENT_MINTED, EVENT_REQUEST_MINT, EVENT_TRANSFERRED,
	},
	HistoryTx, RequestMint,
};

use crate::{
	gafi,
	services::{self, nft_service},
	workers::{EventHandle, HandleParams},
};

async fn on_metadata_set(params: HandleParams<'_>) -> Result<()> {
	let event_parse = params.ev.as_event::<gafi::nfts::events::ItemMetadataSet>()?;
	if let Some(ev) = event_parse {
		let data = String::from_utf8(ev.data.0).ok();
		if let Some(metadata) = data {
			services::nft_service::nft_metadata_set(
				&metadata,
				&ev.collection.to_string(),
				&ev.item.to_string(),
				params.db,
			)
			.await?;
			log::info!(
				"ItemMetadataSet collection {}, token id {}, data: {}",
				ev.collection,
				ev.item,
				metadata
			);
		}
	}
	Ok(())
}

/// Refreshing the supply of an NFT (non-fungible token)
/// when an admin adds more supply.
async fn on_item_added(params: HandleParams<'_>) -> Result<()> {
	let event_parse = params.ev.as_event::<gafi::game::events::ItemAdded>()?;
	if let Some(ev) = event_parse {
		services::nft_service::refresh_supply(ev.collection, ev.item, params.db, params.api)
			.await?;
	}
	Ok(())
}

//ItemCreated
async fn on_item_created(params: HandleParams<'_>) -> Result<()> {
	let event_parse = params.ev.as_event::<gafi::game::events::ItemCreated>()?;
	if let Some(ev) = event_parse {
		services::nft_service::upsert_nft_without_metadata(
			&ev.collection.to_string(),
			&ev.item.to_string(),
			&hex::encode(ev.who.0),
			ev.maybe_supply,
			params.db,
		)
		.await?;
		log::info!("Nft item created {:?}", ev);
	}
	Ok(())
}

async fn on_mint_nft(params: HandleParams<'_>) -> Result<()> {
	let event_parse = params.ev.as_event::<gafi::game::events::Minted>()?;
	if let Some(ev) = event_parse {
		let mut need_refetch_amount = HashMap::<String, u32>::new();
		let mut nfts = vec![];

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

		for item in ev.nfts {
			let key = format!("{}:{}", item.collection, item.item).to_string();
			let amount = need_refetch_amount.get(&key).unwrap_or(&0);
			need_refetch_amount.insert(key, amount + 1);
			/* 	nft_service::upsert_nft_min_price(
				&item.item.to_string(),
				&item.collection.to_string(),
				value_in_decimal,
				params.db,
			)
			.await?; */
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

			services::nft_service::refresh_balance(
				ev.target.clone(),
				collection_id.to_string(),
				token_id.to_string(),
				params.db,
				params.api,
			)
			.await?;
			services::nft_service::refresh_supply(
				collection_id.parse().expect("collection_id must be a number"),
				token_id.parse().expect("token_id must be a number"),
				params.db,
				params.api,
			)
			.await?;
		}

		services::history_service::upsert(
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
		services::history_service::upsert(
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

		services::nft_service::refresh_balance(
			ev.from.clone(),
			ev.collection.to_string(),
			ev.item.to_string(),
			params.db,
			params.api,
		)
		.await?;

		services::nft_service::refresh_balance(
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
		let rq = RequestMint {
			block: params.block.height,
			event_index: params.ev.index(),
			execute_block: ev.block_number,
			extrinsic_index: params.extrinsic_index.unwrap(),
			pool: ev.pool.to_string(),
			target: hex::encode(ev.target.0),
			who: hex::encode(ev.who.0),
		};
		services::nft_service::upsert_request_mint(rq, params.db).await?;
	}
	Ok(())
}

async fn on_nft_metadata_cleared(params: HandleParams<'_>) -> Result<()> {
	let event_parse = params.ev.as_event::<gafi::nfts::events::ItemMetadataCleared>()?;
	if let Some(ev) = event_parse {
		services::nft_service::clear_metadata(
			&ev.collection.to_string(),
			&ev.item.to_string(),
			params.db,
		)
		.await?;
	};
	Ok(())
}

pub fn tasks() -> Vec<EventHandle> {
	vec![
		EventHandle::new(EVENT_MINTED, move |params| Box::pin(on_mint_nft(params))),
		EventHandle::new(EVENT_ITEM_CREATED, move |params| {
			Box::pin(on_item_created(params))
		}),
		EventHandle::new(EVENT_ITEM_ADDED, move |params| {
			Box::pin(on_item_added(params))
		}),
		EventHandle::new(EVENT_ITEM_METADATA_SET, move |params| {
			Box::pin(on_metadata_set(params))
		}),
		EventHandle::new(EVENT_TRANSFERRED, move |params| {
			Box::pin(on_item_transfer(params))
		}),
		EventHandle::new(EVENT_REQUEST_MINT, move |params| {
			Box::pin(on_request_mint(params))
		}),
		EventHandle::new(EVENT_ITEM_METADATA_CLEARED, move |params| {
			Box::pin(on_nft_metadata_cleared(params))
		}),
	]
}
