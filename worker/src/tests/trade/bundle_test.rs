use mongodb::bson::Decimal128;
use shared::{
	constant::{
		EVENT_BUNDLE_BOUGHT, EVENT_SET_BUNDLE, TRADE_SET_BUNDLE, TRADE_STATUS_FOR_SALE,
		TRADE_STATUS_SOLD,
	},
	models, tests,
};

use crate::{
	services::{history_service, trade_service},
	types::{BundleBoughtParams, BundleSetParams},
};

#[tokio::test]
async fn set_bundle() {
	let (mut db_process, db) = tests::utils::get_test_db(10000).await;
	let (_, pk) = tests::utils::mock_account_id32();
	let nft = models::trade::Nft {
		amount: 10,
		collection: 0,
		item: 0,
	};
	let trade_id = 0;
	let price_decimal: Decimal128 = "100".parse().unwrap();
	trade_service::set_bundle(
		BundleSetParams {
			block_height: 0,
			event_index: 0,
			extrinsic_index: 0,
			start_block: Some(1),
			end_block: Some(100),
			nfts: vec![nft.clone()],
			price: Some(price_decimal),
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

	assert_eq!(trade.trade_id, trade_id.to_string());
	assert_eq!(trade.price, Some(price_decimal));
	assert_eq!(trade.bundle, Some(vec![nft.clone()]));
	assert_eq!(trade.owner, pk);
	assert_eq!(trade.start_block, Some(1));
	assert_eq!(trade.end_block, Some(100));
	assert_eq!(trade.status, TRADE_STATUS_FOR_SALE.to_string());
	assert_eq!(trade.trade_type, TRADE_SET_BUNDLE.to_string());

	let history = history_service::get_history_by_trade_id(&trade_id.to_string(), None, &db)
		.await
		.unwrap();
	assert_eq!(history[0].trade_id, Some(trade_id.to_string()));
	assert_eq!(history[0].event_index, 0);
	assert_eq!(history[0].block_height, 0);
	assert_eq!(history[0].extrinsic_index, 0);
	assert_eq!(history[0].nfts, Some(vec![nft]));
	assert_eq!(history[0].price, Some(price_decimal));
	assert_eq!(history[0].event, EVENT_SET_BUNDLE);
	assert_eq!(history[0].trade_type, Some(TRADE_SET_BUNDLE.to_string()));

	let _ = db_process.kill();
}

#[tokio::test]
async fn bought_bundle() {
	let (mut db_process, db) = tests::utils::get_test_db(10000).await;
	let (_, pk) = tests::utils::mock_account_id32();
	let nft = models::trade::Nft {
		amount: 10,
		collection: 0,
		item: 0,
	};
	let trade_id = 0;
	let price_decimal: Decimal128 = "100".parse().unwrap();
	trade_service::set_bundle(
		BundleSetParams {
			block_height: 0,
			event_index: 0,
			extrinsic_index: 0,
			start_block: Some(1),
			end_block: Some(100),
			nfts: vec![nft.clone()],
			price: Some(price_decimal),
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

	let who_bought = "00";

	trade_service::bundle_bought(
		BundleBoughtParams {
			block_height: 1,
			event_index: 0,
			extrinsic_index: 0,
			trade,
			who: who_bought.to_string(),
		},
		&db,
	)
	.await
	.unwrap();

	let trade = trade_service::get_by_trade_id(&db, &trade_id.to_string())
		.await
		.unwrap()
		.unwrap();
	assert_eq!(trade.status, TRADE_STATUS_SOLD.to_string());

	let histories = history_service::get_history_by_trade_id(&trade_id.to_string(), None, &db)
		.await
		.unwrap();
	let history = histories.iter().find(|h| h.event == EVENT_BUNDLE_BOUGHT).unwrap();

	assert_eq!(history.trade_id, Some(trade_id.to_string()));
	assert_eq!(history.block_height, 1);
	assert_eq!(history.event_index, 0);
	assert_eq!(history.extrinsic_index, 0);
	assert_eq!(history.nfts, Some(vec![nft.clone()]));
	assert_eq!(history.price, Some(price_decimal));
	assert_eq!(history.event, EVENT_BUNDLE_BOUGHT);
	assert_eq!(history.trade_type, Some(TRADE_SET_BUNDLE.to_string()));
	assert_eq!(history.from, pk);
	assert_eq!(history.to, Some(who_bought.to_string()));

	let _ = db_process.kill();
}
