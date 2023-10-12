use shared::constant::EVENT_TRADE_CANCELLED;
pub use shared::{
	constant::{TRADE_SET_AUCTION, TRADE_SET_WIST_LIST},
	types::Result,
};

use crate::{
	gafi::{self},
	services::trade_service,
	types::CancelTradeParams,
	workers::{EventHandle, HandleParams},
};

async fn on_trade_cancelled(params: HandleParams<'_>) -> Result<()> {
	let event_parse = params.ev.as_event::<gafi::game::events::TradeCanceled>()?;
	if let Some(ev) = event_parse {
		trade_service::cancel_trade(
			CancelTradeParams {
				block_height: params.block.height,
				event_index: params.ev.index(),
				extrinsic_index: params.extrinsic_index.unwrap(),
				trade_id: ev.trade.to_string(),
				who: hex::encode(ev.who.0),
			},
			params.db,
		)
		.await?;
	}
	Ok(())
}
pub fn tasks() -> Vec<EventHandle> {
	vec![EventHandle::new(EVENT_TRADE_CANCELLED, move |params| {
		Box::pin(on_trade_cancelled(params))
	})]
}
