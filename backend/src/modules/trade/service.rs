use futures_util::TryStreamExt;
use mongodb::{bson::doc, Collection, Database};
use shared::{constant::EMPTY_STR, models, BaseDocument, Trade};

use crate::common::{DBQuery, Page, QueryTrade, TradePage};

use super::dto::TradeDTO;

pub async fn find_trade_by_query(
	params: QueryTrade,
	db: Database,
) -> shared::Result<Option<TradePage>> {
	let col: Collection<Trade> = db.collection(models::trade::Trade::name().as_str());

	let query_find = params.query.to_doc();
	let filter_match = doc! {
		"$match":query_find,
	};

	let paging = doc! {
	  "$facet":{
			"paginatedResults": [ { "$skip": params.skip() }, { "$limit": params.size() } ],
		  "totalCount": [ { "$count": "count" } ],
		},
	};
	let sort = doc! {
		"$sort":params.sort()
	};

	let mut cursor = col.aggregate(vec![filter_match, sort, paging], None).await?;

	let mut list_trades: Vec<TradeDTO> = Vec::new();
	let document = cursor.try_next().await?.ok_or("cursor try_next failed")?;
	let paginated_result = document.get_array("paginatedResults")?;
	paginated_result.into_iter().for_each(|rs| {
		let trade_str = serde_json::to_string(&rs).expect("Failed Parse Trade to String");
		let trade: Trade = serde_json::from_str(&trade_str).expect("Failed to Parse to  Trade");
		list_trades.push(trade.into());
	});
	let count_arr = document.get_array("totalCount")?;
	let count_0 = count_arr.get(0).ok_or("get count");
	let mut count = 0;
	match count_0 {
		Ok(c) => {
			count = c.as_document().ok_or("as document")?.get_i32("count")?;
		},
		Err(_) => {},
	}
	Ok(Some(TradePage {
		message: EMPTY_STR.to_string(),
		data: list_trades,
		page: params.page,
		size: params.size,
		total: count as u64,
	}))
}
