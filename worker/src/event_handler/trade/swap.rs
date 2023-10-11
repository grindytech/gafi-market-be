use mongodb::{
	bson::{doc, Decimal128, Document},
	options::UpdateOptions,
};
use shared::{
	constant::{
		EVENT_SET_SWAP, EVENT_SWAP_CLAIMED, TRADE_SET_SWAP, TRADE_STATUS_FOR_SALE,
		TRADE_STATUS_SOLD,
	},
	models, BaseDocument, Trade,
};
pub use shared::{
	constant::{TRADE_SET_AUCTION, TRADE_SET_WIST_LIST},
	types::Result,
};

use crate::{
	gafi,
	services::{self},
	workers::{EventHandle, HandleParams},
};

async fn on_swap_claimed(params: HandleParams<'_>) -> Result<()> {
	let event_parse = params.ev.as_event::<gafi::game::events::SwapClaimed>()?;
	if let Some(ev) = event_parse {
		let trade_db = params.db.collection::<Trade>(Trade::name().as_str());
		let query = doc! {
		  "trade_id": ev.trade,
		};
		let trade = trade_db.find_one(query.clone(), None).await?.unwrap();
		let update = doc! {
		  "status": TRADE_STATUS_SOLD,
		};
		trade_db.update_one(query.clone(), update, None).await?;
		let config = shared::config::Config::init();
		let maybe_price: Decimal128 = shared::utils::string_decimal_to_number(
			&ev.maybe_bid_price.unwrap_or(0u128).to_string(),
			config.chain_decimal as i32,
		)
		.parse()?;
		let history = models::HistoryTx {
			amount: None,
			block_height: params.block.height,
			event: EVENT_SWAP_CLAIMED.to_string(),
			event_index: params.ev.index(),
			extrinsic_index: params.extrinsic_index.unwrap(),
			from: trade.owner.clone(),
			to: Some(hex::encode(ev.who.0)),
			id: None,
			nfts: trade.maybe_required.clone(),
			source: trade.source.clone(),
			pool: None,
			price: Some(maybe_price),
			trade_id: Some(ev.trade.to_string()),
			trade_type: None,

			value: Some(maybe_price),
		};
		services::history_service::upsert(history, params.db).await?;

		//refresh balance
		if trade.maybe_required.is_some() {
			for nft in trade.maybe_required.unwrap() {
				services::nft_service::refresh_balance(
					ev.who.clone(),
					nft.collection.to_string(),
					nft.item.to_string(),
					params.db,
					params.api,
				)
				.await?;

				let owner_u8 = shared::utils::vec_to_array(hex::decode(trade.owner.clone())?);
				services::nft_service::refresh_balance(
					subxt::utils::AccountId32::from(owner_u8),
					nft.collection.to_string(),
					nft.item.to_string(),
					params.db,
					params.api,
				)
				.await?;
			}
		}
		if trade.source.is_some() {
			for nft in trade.source.unwrap() {
				services::nft_service::refresh_balance(
					ev.who.clone(),
					nft.collection.to_string(),
					nft.item.to_string(),
					params.db,
					params.api,
				)
				.await?;

				let owner_u8 = shared::utils::vec_to_array(hex::decode(trade.owner.clone())?);
				services::nft_service::refresh_balance(
					subxt::utils::AccountId32::from(owner_u8),
					nft.collection.to_string(),
					nft.item.to_string(),
					params.db,
					params.api,
				)
				.await?;
			}
		};
	}

	Ok(())
}

async fn on_swap_set(params: HandleParams<'_>) -> Result<()> {
	let event_parse = params.ev.as_event::<gafi::game::events::SwapSet>()?;
	if let Some(ev) = event_parse {
		let mut source: Vec<models::trade::Nft> = vec![];
		let mut required: Vec<models::trade::Nft> = vec![];
		for nft in ev.source {
			source.push(models::trade::Nft {
				collection: nft.collection,
				item: nft.item,
				amount: nft.amount,
			});
		}
		for nft in ev.required {
			required.push(models::trade::Nft {
				collection: nft.collection,
				item: nft.item,
				amount: nft.amount,
			});
		}
		let maybe_price = match ev.maybe_price {
			Some(p) => {
				let config = shared::config::Config::init();
				let price = shared::utils::string_decimal_to_number(
					&p.to_string(),
					config.chain_decimal as i32,
				);
				Some(price)
			},
			None => None,
		};
		let maybe_price_decimal: Decimal128 = maybe_price.unwrap_or("0".to_string()).parse()?;
		let trade: Document = Trade {
			id: None,
			nft: None,
			maybe_required: Some(required),
			source: Some(source.clone()),
			bundle: None,
			wish_list: None,

			maybe_price: Some(maybe_price_decimal),
			unit_price: None,
			price: None,

			owner: hex::encode(ev.who.0),

			start_block: ev.start_block,
			end_block: ev.end_block,
			duration: None,

			trade_id: ev.trade.to_string(),
			trade_type: TRADE_SET_SWAP.to_string(),

			status: TRADE_STATUS_FOR_SALE.to_string(),
		}
		.into();

		//create sale
		let trade_db = params.db.collection::<Trade>(&Trade::name());
		let options = UpdateOptions::builder().upsert(true).build();
		let query = doc! {
		  "trade_id": ev.trade.to_string(),
		};
		let upsert = doc! {
		  "$set": trade,
		};
		trade_db.update_one(query, upsert, options).await?;

		//refetch balance
		for nft in source {
			services::nft_service::refresh_balance(
				ev.who.clone(),
				nft.collection.to_string(),
				nft.item.to_string(),
				params.db,
				params.api,
			)
			.await?;
		}
	};
	Ok(())
}

pub fn tasks() -> Vec<EventHandle> {
	vec![
		EventHandle::new(EVENT_SET_SWAP, move |params| Box::pin(on_swap_set(params))),
		EventHandle::new(EVENT_SWAP_CLAIMED, move |params| {
			Box::pin(on_swap_claimed(params))
		}),
	]
}
