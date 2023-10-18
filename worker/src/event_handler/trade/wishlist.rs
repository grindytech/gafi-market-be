use mongodb::bson::Decimal128;
use shared::{
	constant::{EVENT_SET_WISH_LIST, EVENT_WIST_LIST_FILLED},
	models,
};
pub use shared::{
	constant::{TRADE_SET_AUCTION, TRADE_SET_WIST_LIST},
	types::Result,
};

use crate::{
	gafi,
	services::{self, trade_service},
	types::{WishlistFilledParams, WishlistSetParams},
	workers::{EventHandle, HandleParams},
};

async fn on_wishlist_filled(params: HandleParams<'_>) -> Result<()> {
	let event_parse = params.ev.as_event::<gafi::game::events::WishlistFilled>()?;
	if let Some(ev) = event_parse {
		let trade = trade_service::get_by_trade_id(params.db, &ev.trade.to_string())
			.await?
			.ok_or("trade not found")?;
		trade_service::wishlist_filled(
			WishlistFilledParams {
				block_height: params.block.height,
				event_index: params.ev.index(),
				extrinsic_index: params.extrinsic_index.unwrap(),
				trade: trade.clone(),
				who: hex::encode(ev.who.0),
			},
			params.db,
		)
		.await?;
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
		trade_service::set_wishlist(
			WishlistSetParams {
				block_height: params.block.height,
				end_block: ev.end_block,
				start_block: ev.start_block,
				event_index: params.ev.index(),
				extrinsic_index: params.extrinsic_index.unwrap(),
				price: Some(price_decimal),
				trade_id: ev.trade.to_string(),
				who: hex::encode(ev.who.0),
				wish_list: wish_list.clone(),
			},
			params.db,
		)
		.await?;
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
