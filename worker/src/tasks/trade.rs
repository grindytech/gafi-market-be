use mongodb::{
	bson::{doc, Decimal128, Document},
	options::UpdateOptions,
};
pub use shared::{
	constant::{TRADE_SET_AUCTION, TRADE_SET_WIST_LIST},
	types::Result,
};
use shared::{
	constant::{
		TRADE_SET_BUNDLE, TRADE_SET_BUY, TRADE_SET_PRICE, TRADE_SET_SWAP, TRADE_STATUS_FOR_SALE,
		TRADE_STATUS_SOLD, EVENT_BOUGHT_ITEM,
	},
	models, BaseDocument, Trade,
};

use crate::{gafi, services, workers::HandleParams};

//game::TradeCanceled
//game::ItemBought
//game::SetBuyClaimed

async fn on_item_bought(params: HandleParams<'_>) -> Result<()> {
	let event_parse = params.ev.as_event::<gafi::game::events::ItemBought>()?;
	if let Some(ev) = event_parse {
		let trade = services::trade::get_by_trade_id(params.db, &ev.trade.to_string())
			.await?
			.unwrap();

		let bundle_of = services::trade::bundle_of(ev.trade, params.api).await?;
		let trade_item = bundle_of.get(0).unwrap();

		let is_sold = trade_item.amount == 0;
		//update history
		let config = shared::config::Config::init();
		let total_value: u128 = ev.bid_unit_price * u128::from(ev.amount);
		let total_value_decimal: Decimal128 = shared::utils::string_decimal_to_number(
			&total_value.to_string(),
			config.chain_decimal as i32,
		)
		.parse()?;
		let mut history_nfts: Vec<models::history_tx::Nft> = vec![];

		let nft = trade.nft.unwrap();
		history_nfts.push(models::history_tx::Nft {
			amount: ev.amount,
			collection_id: nft.collection.to_string(),
			token_id: nft.item.to_string(),
		});
		let extrinsic_index = params.extrinsic_index.unwrap();
		let history = models::HistoryTx {
			block_height: params.block.height,
			event: EVENT_BOUGHT_ITEM.to_string(),
			event_index: params.ev.index(),
			extrinsic_index,
			from: trade.owner,
			to: hex::encode(ev.who.0),
			id: None,
			nfts: history_nfts,
			pool: None,
			tx_hash: None,
			value: total_value_decimal,
		};
		services::history::upsert(history, params.db).await?;

		// update trade
		let trade_db = params.db.collection::<models::Trade>(models::Trade::name().as_str());
		if is_sold {
			trade_db
				.update_one(
					doc! {
						"trade_id": trade.trade_id,
					},
					doc! {
						"$set":{
							"amount": trade_item.amount,
							"status": TRADE_STATUS_SOLD
						}
					},
					None,
				)
				.await?;
		} else {
			trade_db
				.update_one(
					doc! {
						"trade_id": trade.trade_id,
					},
					doc! {
						"$set":{
							"amount": trade_item.amount,
						}
					},
					None,
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

			maybe_price: None,
			unit_price: None,
			price: Some(price_decimal),

			owner: hex::encode(ev.who.0),

			start_block: None, //TODO update event
			end_block: None,
			duration: None,

			trade_id: ev.trade.to_string(),
			trade_type: TRADE_SET_WIST_LIST.to_string(),

			sold: None,
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
	};
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
				collection: nft.collection,
				item: nft.item,
				amount: nft.amount,
			});
		}
		let trade: Document = Trade {
			id: None,
			nft: None,
			maybe_required: None,
			source: Some(source.clone()),
			bundle: None,
			wish_list: None,

			maybe_price: Some(maybe_price_decimal),
			unit_price: None,
			price: None,

			owner: hex::encode(ev.who.0),

			start_block: None, //TODO update event
			end_block: None,
			duration: Some(ev.duration),

			trade_id: ev.trade.to_string(),
			trade_type: TRADE_SET_AUCTION.to_string(),

			sold: None,
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
			services::nft::refresh_balance(
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

async fn on_bundle_set(params: HandleParams<'_>) -> Result<()> {
	let event_parse = params.ev.as_event::<gafi::game::events::BundleSet>()?;
	if let Some(ev) = event_parse {
		let mut bundle: Vec<models::trade::Nft> = vec![];
		for nft in ev.bundle {
			bundle.push(models::trade::Nft {
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
			bundle: Some(bundle.clone()),
			wish_list: None,

			maybe_price: None,
			unit_price: None,
			price: Some(price_decimal),

			owner: hex::encode(ev.who.0),

			start_block: None, //TODO update event
			end_block: None,
			duration: None,

			trade_id: ev.trade.to_string(),
			trade_type: TRADE_SET_BUNDLE.to_string(),

			sold: None,
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
		for nft in bundle {
			services::nft::refresh_balance(
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

async fn on_swap_buy(params: HandleParams<'_>) -> Result<()> {
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

			start_block: None, //TODO update event
			end_block: None,
			duration: None,

			trade_id: ev.trade.to_string(),
			trade_type: TRADE_SET_SWAP.to_string(),

			sold: None,
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
			services::nft::refresh_balance(
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

//game::BuySet
async fn on_set_buy(params: HandleParams<'_>) -> Result<()> {
	let event_parse = params.ev.as_event::<gafi::game::events::BuySet>()?;
	if let Some(ev) = event_parse {
		let config = shared::config::Config::init();
		let unit_price = shared::utils::string_decimal_to_number(
			&ev.unit_price.to_string(),
			config.chain_decimal as i32,
		);
		let unit_price_decimal: Decimal128 = unit_price.parse()?;
		let trade: Document = Trade {
			id: None,

			nft: Some(models::trade::Nft {
				amount: ev.amount,
				collection: ev.collection,
				item: ev.item,
			}),
			maybe_required: None,
			source: None,
			bundle: None,
			wish_list: None,

			maybe_price: None,
			unit_price: Some(unit_price_decimal),
			price: None,

			owner: hex::encode(ev.who.0),

			start_block: None, //TODO update event
			end_block: None,
			duration: None,

			trade_id: ev.trade.to_string(),
			trade_type: TRADE_SET_BUY.to_string(),

			sold: Some(0),
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
	}
	Ok(())
}

//game::PriceSet
async fn on_set_price(params: HandleParams<'_>) -> Result<()> {
	let event_parse = params.ev.as_event::<gafi::game::events::PriceSet>()?;
	if let Some(ev) = event_parse {
		let config = shared::config::Config::init();
		let unit_price = shared::utils::string_decimal_to_number(
			&ev.unit_price.to_string(),
			config.chain_decimal as i32,
		);
		let unit_price_decimal: Decimal128 = unit_price.parse()?;
		let trade: Document = Trade {
			id: None,

			nft: Some(models::trade::Nft {
				amount: ev.amount,
				collection: ev.collection,
				item: ev.item,
			}),
			maybe_required: None,
			source: None,
			bundle: None,
			wish_list: None,

			owner: hex::encode(ev.who.0),
			start_block: None,
			end_block: None,
			duration: None,

			unit_price: Some(unit_price_decimal),
			maybe_price: None,
			price: None,

			trade_id: ev.trade.to_string(),
			trade_type: TRADE_SET_PRICE.to_string(),

			sold: Some(0),
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
		services::nft::refresh_balance(
			ev.who,
			ev.collection.to_string(),
			ev.item.to_string(),
			params.db,
			params.api,
		)
		.await?;
	}
	Ok(())
}
