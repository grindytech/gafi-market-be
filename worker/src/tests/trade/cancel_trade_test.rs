use mongodb::bson::Decimal128;
use shared::{
	constant::{EVENT_TRADE_CANCELLED, TRADE_STATUS_CANCELED, TRADE_SET_PRICE},
	tests, models,
};

use crate::{
	services::{history_service, trade_service},
	types::{CancelTradeParams, SetPriceParams},
};
#[tokio::test]
pub async fn cancel_sale() {
	let (mut db_process, db) = tests::utils::get_test_db(10000).await;
	let (_, pk) = tests::utils::mock_account_id32();
	let nft = models::trade::Nft {
		amount: 1,
		collection: 0,
		item: 0,
	};
	let unit_price_decimal: Decimal128 = "100".parse().unwrap();
	let trade_id = 0;
	let set_price_params = SetPriceParams {
		block_height: 0,
		event_index: 0,
		extrinsic_index: 0,
		nft: nft.clone(),
		trade_id: trade_id.to_string(),
		unit_price: unit_price_decimal,
		who: pk.clone(),
		end_block: Some(100),
		start_block: Some(1),
	};
	trade_service::set_price(set_price_params, &db).await.unwrap();
	trade_service::cancel_trade(
		CancelTradeParams {
			block_height: 1,
			event_index: 0,
			extrinsic_index: 0,
			trade_id: trade_id.to_string(),
			who: pk.clone(),
		},
		&db,
	)
	.await
	.unwrap();

	let trade = trade_service::get_by_trade_id(&db, &trade_id.to_string())
		.await
		.unwrap()
		.unwrap();

	assert_eq!(trade.status, TRADE_STATUS_CANCELED.to_string());

	let histories = history_service::get_history_by_trade_id(&trade_id.to_string(), None, &db)
		.await
		.unwrap();
	let history = histories.iter().find(|h| h.event == EVENT_TRADE_CANCELLED).unwrap();
	assert_eq!(history.trade_id, Some(trade_id.to_string()));
	assert_eq!(history.event_index, 0);
	assert_eq!(history.block_height, 1);
	assert_eq!(history.extrinsic_index, 0);
	assert_eq!(history.nfts, Some(vec![nft.clone()]));
	assert_eq!(history.event, EVENT_TRADE_CANCELLED);
	assert_eq!(history.trade_type, Some(TRADE_SET_PRICE.to_string()));
	assert_eq!(history.from, pk);

	let _ = db_process.kill();
}
