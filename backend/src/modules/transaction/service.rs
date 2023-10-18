use futures_util::TryStreamExt;
use mongodb::{bson::doc, Collection, Database};
use shared::{constant::EMPTY_STR, history_tx::HistoryTx, models, BaseDocument};

use crate::common::{utils::get_total_page, DBQuery, Page, QueryTx};

use super::dto::HistoryTxDTO;

pub async fn find_tx_by_hash(
	tx_hash: &String,
	db: Database,
) -> Result<Option<HistoryTxDTO>, mongodb::error::Error> {
	let col: Collection<HistoryTx> = db.collection(models::history_tx::HistoryTx::name().as_str());
	let filter = doc! {"tx_hash": tx_hash};
	if let Ok(Some(tx_detail)) = col.find_one(filter, None).await {
		Ok(Some(tx_detail.into()))
	} else {
		Ok(None)
	}
}
pub async fn find_tx_by_query(
	params: QueryTx,
	db: Database,
) -> shared::Result<Option<Page<HistoryTxDTO>>> {
	let col: Collection<HistoryTx> = db.collection(models::history_tx::HistoryTx::name().as_str());

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
	let mut list_transactions: Vec<HistoryTxDTO> = Vec::new();
	let document = cursor.try_next().await?.ok_or("cursor try_next failed")?;
	let paginated_result = document.get_array("paginatedResults")?;
	paginated_result.into_iter().for_each(|rs| {
		let history_str = serde_json::to_string(&rs).expect("Failed Parse History to String");
		let transaction: HistoryTx =
			serde_json::from_str(&history_str).expect("Failed to Parse to  History");
		list_transactions.push(transaction.into());
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
	Ok(Some(Page::<HistoryTxDTO> {
		data: list_transactions,
		message: EMPTY_STR.to_string(),
		page: params.page,
		size: params.size,
		total: count as u64,
	}))
}
