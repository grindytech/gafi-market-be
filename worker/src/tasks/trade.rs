pub use shared::types::Result;

use crate::{gafi, workers::HandleParams};

//game::PriceSet
//game::TradeCanceled
//game::ItemBought
//game::BuySet
//game::SetBuyClaimed

async fn on_set_price(params: HandleParams<'_>) -> Result<()> {
	let event_parse = params.ev.as_event::<gafi::game::events::PriceSet>()?;
	if let Some(ev) = event_parse {
    //create sale
    //refetch balance
  }
	Ok(())
}
