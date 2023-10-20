use mongodb::bson::Decimal128;
use shared::{
	constant::{EVENT_AUCTION_CLAIMED, EVENT_BID, EVENT_SET_AUCTION},
	models,
	utils::string_decimal_to_number,
};
pub use shared::{
	constant::{TRADE_SET_AUCTION, TRADE_SET_WIST_LIST},
	types::Result,
};

use crate::{
	gafi,
	services::{self, trade_service},
	types::{AuctionBidParams, AuctionClaimParams, AuctionSetParams},
	workers::{EventHandle, HandleParams},
};

async fn on_auction_claimed(params: HandleParams<'_>) -> Result<()> {
	let event_parse = params.ev.as_event::<gafi::game::events::AuctionClaimed>()?;
	if let Some(ev) = event_parse {
		let trade = trade_service::get_trade_by_trade_id(&ev.trade.to_string(), params.db)
			.await
			.unwrap()
			.expect("Trade should be found");

		let mut who = None;
		let mut to = None;
		let mut ask_price = None;
		match ev.maybe_bid {
			Some((account, p)) => {
				who = Some(account.clone());
				ask_price = Some(p);
				to = Some(hex::encode(account.0));
			},
			None => {},
		};
		let price = trade.price;
		let config = shared::config::Config::init();
		trade_service::auction_claim(
			AuctionClaimParams {
				block_height: params.block.height,
				event_index: params.ev.index(),
				extrinsic_index: params.extrinsic_index.unwrap(),
				from: trade.owner.clone(),
				to,
				nfts: trade.bundle.clone(),
				price,
				trade_id: trade.trade_id,
				trade_type: trade.trade_type,
				ask_price: Some(
					shared::utils::string_decimal_to_number(
						&ask_price.unwrap_or(0u128).to_string(),
						config.chain_decimal as i32,
					)
					.parse()?,
				),
			},
			params.db,
		)
		.await?;

		for nft in trade.source.expect("on_auction_claimed trade.source fail") {
			if who.is_some() {
				services::nft_service::refresh_balance(
					who.clone().expect("on_auction_claimed trade.source who"),
					nft.collection.to_string(),
					nft.item.to_string(),
					params.db,
					params.api,
				)
				.await?;
			};
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

async fn on_auction_set(params: HandleParams<'_>) -> Result<()> {
	let event_parse = params.ev.as_event::<gafi::game::events::AuctionSet>()?;
	if let Some(ev) = event_parse {
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
		let mut source: Vec<models::trade::Nft> = vec![];
		for nft in ev.source {
			source.push(models::trade::Nft {
				collection: nft.collection.to_string(),
				item: nft.item.to_string(),
				amount: nft.amount,
			});
		}
		trade_service::auction_set(
			AuctionSetParams {
				duration: ev.duration,
				maybe_price: maybe_price_decimal,
				owner: hex::encode(ev.who.0).to_string(),
				start_block: ev.start_block,
				source: source.clone(),
				trade_id: ev.trade.to_string(),
				block_height: params.block.height,
				event_index: params.ev.index(),
				extrinsic_index: params.extrinsic_index.unwrap(),
			},
			params.db,
		)
		.await?;

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
	}
	Ok(())
}

async fn on_auction_bid(params: HandleParams<'_>) -> Result<()> {
	let event_parse = params.ev.as_event::<gafi::game::events::Bid>()?;
	if let Some(ev) = event_parse {
		let trade = trade_service::get_trade_by_trade_id(&ev.trade.to_string(), params.db)
			.await?
			.ok_or("trade not found")?;

		let config = shared::config::Config::init();
		let bid_price = string_decimal_to_number(&ev.bid.to_string(), config.chain_decimal as i32);
		let bid_price_decimal128: Decimal128 = bid_price.parse()?;
		trade_service::create_auction_bid(
			AuctionBidParams {
				bid: bid_price_decimal128,
				block_height: params.block.height,
				event_index: params.ev.index(),
				extrinsic_index: params.extrinsic_index.unwrap(),
				trade,
				who: hex::encode(ev.who.0).to_string(),
			},
			params.db,
		)
		.await?;
	}
	Ok(())
}

pub fn tasks() -> Vec<EventHandle> {
	vec![
		EventHandle::new(EVENT_SET_AUCTION, move |params| {
			Box::pin(on_auction_set(params))
		}),
		EventHandle::new(EVENT_AUCTION_CLAIMED, move |params| {
			Box::pin(on_auction_claimed(params))
		}),
		EventHandle::new(EVENT_BID, move |params| Box::pin(on_auction_bid(params))),
	]
}
