use mongodb::bson::Decimal128;
use shared::{
	constant::{EVENT_SWAP_CLAIMED, TRADE_SET_SWAP, TRADE_STATUS_FOR_SALE, TRADE_STATUS_SOLD},
	models, tests,
};

use crate::{
	services::{
		history_service,
		trade_service::{self, get_trade_by_trade_id},
	},
	types::{SwapClaimedParams, SwapSetParams},
};

#[tokio::test]
pub async fn set_and_claim_swap() {
	let (mut db_process, db) = tests::utils::get_test_db(10000).await;
	let (_, pk) = tests::utils::mock_account_id32();
	let nft = models::trade::Nft {
		amount: 10,
		collection: "0".to_string(),
		item: "0".to_string(),
	};
	let nft1 = models::trade::Nft {
		amount: 10,
		collection: "0".to_string(),
		item: "1".to_string(),
	};
	let maybe_price: Decimal128 = "10".parse().unwrap();
	let trade_id = 0;
	trade_service::set_swap(
		SwapSetParams {
			block_height: 0,
			event_index: 0,
			start_block: Some(1),
			end_block: Some(100),
			extrinsic_index: 0,
			price: Some(maybe_price),
			required: vec![nft1.clone()],
			source: vec![nft.clone()],
			trade_id: trade_id.to_string(),
			who: pk.clone(),
		},
		&db,
	)
	.await
	.unwrap();

	let trade = get_trade_by_trade_id(&trade_id.to_string(), &db).await.unwrap().unwrap();
	assert_eq!(trade.owner, pk);
	assert_eq!(trade.price, Some(maybe_price));
	assert_eq!(trade.source, Some(vec![nft.clone()]));
	assert_eq!(trade.maybe_required, Some(vec![nft1.clone()]));
	assert_eq!(trade.trade_type, TRADE_SET_SWAP.to_string());
	assert_eq!(trade.status, TRADE_STATUS_FOR_SALE.to_string());
	assert_eq!(trade.start_block, Some(1));
	assert_eq!(trade.end_block, Some(100));

	let who_claimed = "00000";
	trade_service::claim_swap(
		SwapClaimedParams {
			block_height: 1,
			event_index: 0,
			extrinsic_index: 0,
			trade,
			who: who_claimed.to_string(),
		},
		&db,
	)
	.await
	.unwrap();

	let trade = get_trade_by_trade_id(&trade_id.to_string(), &db).await.unwrap().unwrap();
	assert_eq!(trade.status, TRADE_STATUS_SOLD.to_string());

	let histories = history_service::get_history_by_trade_id(&trade_id.to_string(), None, &db)
		.await
		.unwrap();

	let history = histories.iter().find(|h| h.event == EVENT_SWAP_CLAIMED).unwrap();
	assert_eq!(history.trade_id, Some(trade_id.to_string()));
	assert_eq!(history.block_height, 1);
	assert_eq!(history.event_index, 0);
	assert_eq!(history.extrinsic_index, 0);
	assert_eq!(history.nfts, Some(vec![nft1.clone()]));
	assert_eq!(history.source, Some(vec![nft.clone()]));
	assert_eq!(history.price, Some(maybe_price));
	assert_eq!(history.event, EVENT_SWAP_CLAIMED);
	assert_eq!(history.trade_type, Some(TRADE_SET_SWAP.to_string()));
	assert_eq!(history.from, pk);
	assert_eq!(history.to, Some(who_claimed.to_string()));

	let _ = db_process.kill();
}
