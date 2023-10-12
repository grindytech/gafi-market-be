use mongodb::{bson::Decimal128, Database};
use shared::{
	constant::{EVENT_AUCTION_CLAIMED, TRADE_STATUS_SOLD},
	models, tests, HistoryTx, Trade,
};

use crate::{services::{
	history_service,
	trade_service,
}, types::{AuctionSetParams, AuctionClaimParams}};

async fn do_auction(db: &Database) {
	let (_, public_key) = tests::utils::mock_account_id32();
	let nfts: Vec<models::trade::Nft> = vec![models::trade::Nft {
		amount: 1,
		collection: 0,
		item: 0,
	}];
	trade_service::auction_set(
		AuctionSetParams {
			duration: 1000,
			maybe_price: Decimal128::from("1000".to_string().parse().unwrap()),
			owner: public_key,
			trade_id: "0".to_string(),
			source: nfts.clone(),
			start_block: Some(0),
			block_height: 0,
			event_index: 0,
			extrinsic_index: 0,
		},
		&db,
	)
	.await
	.unwrap();
}

#[tokio::test]
async fn create_auction_test() {
	let trade_test_json = r#"{
    "_id": null,
    "trade_id": "0",
    "trade_type": "SetAuction",
    "owner": "ec84321d9751c066fb923035073a73d467d44642c457915e7496c52f45db1f65",
    "start_block": 0,
    "end_block": null,
    "duration": 1000,
    "price": {
      "$numberDecimal": "1000"
    },
    "nft": null,
    "source": [
      {
        "collection": 0,
        "item": 0,
        "amount": 1
      }
    ],
    "maybe_required": null,
    "bundle": null,
    "wish_list": null,
    "sold": null,
    "status": "ForSale",
		"highest_bid": null
  }"#;
	let set_auction_history_json = r#"{
    "_id": null,
    "tx_hash": null,
    "extrinsic_index": 0,
    "event_index": 0,
    "block_height": 0,
    "value": null,
    "event": "Game:AuctionSet",
    "from": "ec84321d9751c066fb923035073a73d467d44642c457915e7496c52f45db1f65",
    "to": null,
    "pool": null,
    "nfts": null,
    "amount": null,
    "price": {
      "$numberDecimal": "1000"
    },
    "trade_id": "0",
    "trade_type": "SetAuction",
    "source": [
      {
        "collection": 0,
        "item": 0,
        "amount": 1
      }
    ]
  }"#;

	let mut mock_trade = serde_json::from_str::<Trade>(trade_test_json).unwrap();
	let mut mock_set_auction_history =
		serde_json::from_str::<HistoryTx>(set_auction_history_json).unwrap();

	let (mut db_process, db) = tests::utils::get_test_db(10000).await;
	do_auction(&db).await;

	let trade = trade_service::get_by_trade_id(&db, "0").await.unwrap().unwrap();
	let histories = history_service::get_history_by_trade_id("0", None, &db).await.unwrap();

	mock_trade.id = trade.id;
	assert_eq!(mock_trade, trade);

	let history = histories.get(0).unwrap();
	mock_set_auction_history.id = history.id;
	assert_eq!(mock_set_auction_history, *history);

	let _ = db_process.kill();
}

#[tokio::test]
async fn claim_auction_test() {
	let (mut db_process, db) = tests::utils::get_test_db(10000).await;
	do_auction(&db).await;
	let trade = trade_service::get_by_trade_id(&db, "0").await.unwrap().unwrap();
	let (_, public_key) = tests::utils::mock_account_id32();

	let params = AuctionClaimParams {
		trade_id: trade.trade_id,
		trade_type: trade.trade_type,
		from: trade.owner,
		to: Some(public_key),
		price: trade.price,
		block_height: 1,
		event_index: 0,
		extrinsic_index: 0,
		nfts: trade.source,
		ask_price: trade.price,
	};

	trade_service::auction_claim(params, &db).await.unwrap();
	let trade = trade_service::get_by_trade_id(&db, "0").await.unwrap().unwrap();
	let histories = history_service::get_history_by_trade_id("0", None, &db).await.unwrap();

	assert_eq!(trade.status, TRADE_STATUS_SOLD);
	let history = histories
		.clone()
		.into_iter()
		.find(move |h| h.event == EVENT_AUCTION_CLAIMED.to_string());
	assert!(histories.len() == 2 && history.is_some());

	let _ = db_process.kill();
}


