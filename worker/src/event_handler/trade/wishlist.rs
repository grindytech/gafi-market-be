use mongodb::{
	bson::{doc, Decimal128, Document},
	options::UpdateOptions,
};
use shared::{
	constant::{
		EVENT_SET_WISH_LIST, EVENT_WIST_LIST_FILLED, TRADE_STATUS_FOR_SALE, TRADE_STATUS_SOLD,
	},
	history_tx, models, BaseDocument, Trade,
};
pub use shared::{
	constant::{TRADE_SET_AUCTION, TRADE_SET_WIST_LIST},
	types::Result,
};

use crate::{
	gafi, services,
	workers::{EventHandle, HandleParams},
};

async fn on_wishlist_filled(params: HandleParams<'_>) -> Result<()> {
	let event_parse = params.ev.as_event::<gafi::game::events::WishlistFilled>()?;
	if let Some(ev) = event_parse {
		let trade_db = params.db.collection::<Trade>(Trade::name().as_str());
		let query = doc! {
		  "trade_id": ev.trade,
		};
		let trade = trade_db.find_one(query.clone(), None).await?.unwrap();
		let update = doc! {
		  "status": TRADE_STATUS_SOLD,
		};
		let config = shared::config::Config::init();
		trade_db.update_one(query.clone(), update, None).await?;
		let history = history_tx::HistoryTx {
			id: None,
			amount: None,
			price: trade.price,
			block_height: params.block.height,
			event: EVENT_WIST_LIST_FILLED.to_string(),
			event_index: params.ev.index(),
			extrinsic_index: params.extrinsic_index.unwrap(),
			from: trade.owner.clone(),
			to: Some(hex::encode(ev.who.0)),
			nfts: trade.wish_list.clone(),
			pool: None,
			source: None,
			trade_id: Some(trade.trade_id),
			trade_type: Some(trade.trade_type),
			tx_hash: None,
			value: Some(
				shared::utils::string_decimal_to_number(
					&ev.ask_price.to_string(),
					config.chain_decimal as i32,
				)
				.parse()?,
			),
		};
		services::history_service::upsert(history, params.db).await?;
		for nft in trade.wish_list.unwrap() {
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
	Ok(())
}

async fn on_wishlist_set(params: HandleParams<'_>) -> Result<()> {
	let event_parse = params.ev.as_event::<gafi::game::events::WishlistSet>()?;
	if let Some(ev) = event_parse {
		let mut wish_list: Vec<models::trade::Nft> = vec![];
		for nft in ev.wishlist {
			wish_list.push(models::trade::Nft {
				collection: nft.collection,
				item: nft.item,
				amount: nft.amount,
			});
		}
		let config = shared::config::Config::init();
		let price = shared::utils::string_decimal_to_number(
			&ev.price.to_string(),
			config.chain_decimal as i32,
		);
		let price_decimal: Decimal128 = price.parse()?;
		let trade: Document = Trade {
			id: None,
			nft: None,
			maybe_required: None,
			source: None,
			bundle: None,
			wish_list: Some(wish_list),

			price: Some(price_decimal),

			owner: hex::encode(ev.who.0),

			start_block: ev.start_block,
			end_block: ev.end_block,
			duration: None,

			trade_id: ev.trade.to_string(),
			trade_type: TRADE_SET_WIST_LIST.to_string(),

			status: TRADE_STATUS_FOR_SALE.to_string(),
			highest_bid: None,
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
	};
	Ok(())
}

pub fn tasks() -> Vec<EventHandle> {
	vec![
		EventHandle::new(EVENT_SET_WISH_LIST, move |params| {
			Box::pin(on_wishlist_set(params))
		}),
		EventHandle::new(EVENT_WIST_LIST_FILLED, move |params| {
			Box::pin(on_wishlist_filled(params))
		}),
	]
}
