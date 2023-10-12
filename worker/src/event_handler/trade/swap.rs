use mongodb::bson::Decimal128;
pub use shared::Result;
use shared::{
	constant::{EVENT_SET_SWAP, EVENT_SWAP_CLAIMED},
	models,
};

use crate::{
	gafi,
	services::{self, trade_service},
	types::SwapSetParams,
	workers::{EventHandle, HandleParams},
};

async fn on_swap_claimed(params: HandleParams<'_>) -> Result<()> {
	let event_parse = params.ev.as_event::<gafi::game::events::SwapClaimed>()?;
	if let Some(ev) = event_parse {
		let trade = trade_service::get_trade_by_trade_id(&ev.trade.to_string(), params.db)
			.await?
			.ok_or("trade not found")?;
		trade_service::claim_swap(
			crate::types::SwapClaimedParams {
				block_height: params.block.height,
				event_index: params.ev.index(),
				extrinsic_index: params.extrinsic_index.unwrap(),
				who: hex::encode(ev.who.0),
				trade: trade.clone(),
			},
			params.db,
		)
		.await?;

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

		trade_service::set_swap(
			SwapSetParams {
				block_height: params.block.height,
				start_block: ev.start_block,
				end_block: ev.end_block,
				event_index: params.ev.index(),
				extrinsic_index: params.extrinsic_index.unwrap(),
				price: Some(maybe_price_decimal),
				required,
				source: source.clone(),
				trade_id: ev.trade.to_string(),
				who: hex::encode(ev.who.0),
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
