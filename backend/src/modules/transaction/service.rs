use futures_util::TryStreamExt;
use mongodb::{bson::doc, Collection, Database};
use shared::{constant::EMPTY_STR, models, transaction::Transaction};

use crate::common::{
	utils::{get_filter_option, get_total_page},
	DBQuery, Page, QueryPage,
};

use super::dto::{QueryFindHistory, TransactionDTO};

pub async fn find_tx_by_hash(
	tx_hash: &String,
	db: Database,
) -> Result<Option<TransactionDTO>, mongodb::error::Error> {
	let col: Collection<Transaction> = db.collection(models::nft::NAME);
	let filter = doc! {"tx_hash": tx_hash};
	if let Ok(Some(tx_detail)) = col.find_one(filter, None).await {
		Ok(Some(tx_detail.into()))
	} else {
		Ok(None)
	}
}
pub async fn find_tx_by_query(
	params: QueryPage<QueryFindHistory>,
	db: Database,
) -> Result<Option<Page<TransactionDTO>>, mongodb::error::Error> {
	let col: Collection<Transaction> = db.collection(models::transaction::NAME);

	let query_find = params.query.to_doc();
	let filter_option = get_filter_option(params.order_by, params.desc).await;
	let mut cursor = col.find(query_find, filter_option).await?;

	let mut list_transactions: Vec<TransactionDTO> = Vec::new();
	while let Some(tx) = cursor.try_next().await? {
		list_transactions.push(tx.into())
	}
	let total = get_total_page(list_transactions.len(), params.size).await;
	Ok(Some(Page::<TransactionDTO> {
		data: list_transactions,
		message: EMPTY_STR.to_string(),
		page: params.page,
		size: params.size,
		total,
	}))
}
