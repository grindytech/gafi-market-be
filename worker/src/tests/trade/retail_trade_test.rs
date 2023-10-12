use mongodb::bson::Decimal128;
use shared::{
	constant::{
		EVENT_BOUGHT_ITEM, EVENT_SET_BUY, EVENT_SET_PRICE, TRADE_SET_BUY, TRADE_SET_PRICE,
		TRADE_STATUS_FOR_SALE, TRADE_STATUS_SOLD,
	},
	models, tests,
};

use crate::{
	services::{history_service, trade_service},
	types::{ItemBoughtParams, SetPriceParams},
};

// on set price
// - create trade
// - create history
#[tokio::test]
async fn set_price() {
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
	let _ = trade_service::set_price(set_price_params, &db).await.unwrap();

	let trade = trade_service::get_by_trade_id(&db, &trade_id.to_string())
		.await
		.unwrap()
		.unwrap();

	let history = history_service::get_history_by_trade_id(&trade_id.to_string(), None, &db)
		.await
		.unwrap();

	assert_eq!(trade.trade_id, trade_id.to_string());
	assert_eq!(trade.price, Some(unit_price_decimal));
	assert_eq!(trade.nft, Some(nft.clone()));
	assert_eq!(trade.owner, pk);
	assert_eq!(trade.start_block, Some(1));
	assert_eq!(trade.end_block, Some(100));
	assert_eq!(trade.status, TRADE_STATUS_FOR_SALE.to_string());
	assert_eq!(trade.trade_type, TRADE_SET_PRICE.to_string());

	assert_eq!(history[0].trade_id, Some(trade_id.to_string()));
	assert_eq!(history[0].event_index, 0);
	assert_eq!(history[0].block_height, 0);
	assert_eq!(history[0].extrinsic_index, 0);
	assert_eq!(history[0].nfts, Some(vec![nft]));
	assert_eq!(history[0].price, Some(unit_price_decimal));
	assert_eq!(history[0].event, EVENT_SET_PRICE);
	assert_eq!(history[0].trade_type, Some(TRADE_SET_PRICE.to_string()));

	let _ = db_process.kill();
}

//on set by
// - create trade
// - create history
#[tokio::test]
async fn set_buy() {
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
	let _ = trade_service::set_buy(set_price_params, &db).await.unwrap();

	let trade = trade_service::get_by_trade_id(&db, &trade_id.to_string())
		.await
		.unwrap()
		.unwrap();

	let history = history_service::get_history_by_trade_id(&trade_id.to_string(), None, &db)
		.await
		.unwrap();

	assert_eq!(trade.trade_id, trade_id.to_string());
	assert_eq!(trade.price, Some(unit_price_decimal));
	assert_eq!(trade.nft, Some(nft.clone()));
	assert_eq!(trade.owner, pk);
	assert_eq!(trade.start_block, Some(1));
	assert_eq!(trade.end_block, Some(100));
	assert_eq!(trade.status, TRADE_STATUS_FOR_SALE.to_string());
	assert_eq!(trade.trade_type, TRADE_SET_BUY.to_string());

	assert_eq!(history[0].trade_id, Some(trade_id.to_string()));
	assert_eq!(history[0].event_index, 0);
	assert_eq!(history[0].block_height, 0);
	assert_eq!(history[0].extrinsic_index, 0);
	assert_eq!(history[0].nfts, Some(vec![nft]));
	assert_eq!(history[0].price, Some(unit_price_decimal));
	assert_eq!(history[0].event, EVENT_SET_BUY);
	assert_eq!(history[0].trade_type, Some(TRADE_SET_BUY.to_string()));

	let _ = db_process.kill();
}

//on set item bought
// - update trade status
// - create history
#[tokio::test]
async fn set_price_item_bought() {
	let (mut db_process, db) = tests::utils::get_test_db(10000).await;
	let (_, pk) = tests::utils::mock_account_id32();
	let nft = models::trade::Nft {
		amount: 10,
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
	let _ = trade_service::set_price(set_price_params, &db).await.unwrap();
	let bought_pk = "000000000";
	//buy one
	let _ = trade_service::bought_item(
		ItemBoughtParams {
			amount: 1,
			block_height: 1,
			event_index: 1,
			extrinsic_index: 1,
			is_sold: false,
			nft: nft.clone(),
			trade_id: trade_id.to_string(),
			who: bought_pk.to_string(),
		},
		&db,
	)
	.await
	.unwrap();

	let histories = history_service::get_history_by_trade_id(&trade_id.to_string(), None, &db)
		.await
		.unwrap();
	let bought_history = histories.iter().find(|h| h.event == EVENT_BOUGHT_ITEM).unwrap();
	assert_eq!(bought_history.trade_id, Some(trade_id.to_string()));
	assert_eq!(bought_history.event_index, 1);
	assert_eq!(bought_history.block_height, 1);
	assert_eq!(bought_history.extrinsic_index, 1);
	assert_eq!(bought_history.nfts, Some(vec![nft.clone()]));
	assert_eq!(bought_history.price, Some(unit_price_decimal));
	assert_eq!(bought_history.event, EVENT_BOUGHT_ITEM);
	assert_eq!(bought_history.trade_type, Some(TRADE_SET_PRICE.to_string()));
	assert_eq!(bought_history.amount, Some(1));
	assert_eq!(bought_history.from, pk);
	assert_eq!(bought_history.to, Some(bought_pk.to_string()));

	let _ = trade_service::bought_item(
		ItemBoughtParams {
			amount: 9,
			block_height: 2,
			event_index: 1,
			extrinsic_index: 1,
			is_sold: true,
			nft: nft.clone(),
			trade_id: trade_id.to_string(),
			who: bought_pk.to_string(),
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

	let _ = db_process.kill();
}

//on set item bought
// - update trade status
// - create history
#[tokio::test]
async fn set_buy_item_bought() {
	let (mut db_process, db) = tests::utils::get_test_db(10000).await;
	let (_, pk) = tests::utils::mock_account_id32();
	let nft = models::trade::Nft {
		amount: 10,
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
	let _ = trade_service::set_buy(set_price_params, &db).await.unwrap();
	let bought_pk = "000000000";
	//buy one
	let _ = trade_service::bought_item(
		ItemBoughtParams {
			amount: 1,
			block_height: 1,
			event_index: 1,
			extrinsic_index: 1,
			is_sold: false,
			nft: nft.clone(),
			trade_id: trade_id.to_string(),
			who: bought_pk.to_string(),
		},
		&db,
	)
	.await
	.unwrap();

	let histories = history_service::get_history_by_trade_id(&trade_id.to_string(), None, &db)
		.await
		.unwrap();
	let bought_history = histories.iter().find(|h| h.event == EVENT_BOUGHT_ITEM).unwrap();
	assert_eq!(bought_history.trade_id, Some(trade_id.to_string()));
	assert_eq!(bought_history.event_index, 1);
	assert_eq!(bought_history.block_height, 1);
	assert_eq!(bought_history.extrinsic_index, 1);
	assert_eq!(bought_history.nfts, Some(vec![nft.clone()]));
	assert_eq!(bought_history.price, Some(unit_price_decimal));
	assert_eq!(bought_history.event, EVENT_BOUGHT_ITEM);
	assert_eq!(bought_history.trade_type, Some(TRADE_SET_BUY.to_string()));
	assert_eq!(bought_history.amount, Some(1));
	assert_eq!(bought_history.from, bought_pk.to_string());
	assert_eq!(bought_history.to, Some(pk));

	let _ = trade_service::bought_item(
		ItemBoughtParams {
			amount: 9,
			block_height: 2,
			event_index: 1,
			extrinsic_index: 1,
			is_sold: true,
			nft: nft.clone(),
			trade_id: trade_id.to_string(),
			who: bought_pk.to_string(),
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

	let _ = db_process.kill();
}
