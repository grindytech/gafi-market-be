use futures_util::TryStreamExt;
use mongodb::{Collection, Database};
use shared::{constant::EMPTY_STR, models, BaseDocument, Trade};

use crate::common::{
	utils::{get_filter_option, get_total_page},
	DBQuery, Page, QueryPage,
};

use super::dto::{QueryFindTrade, TradeDTO};

pub async fn find_trades_by_query(
	params: QueryPage<QueryFindTrade>,
	db: Database,
) -> Result<Option<Page<TradeDTO>>, mongodb::error::Error> {
	let col: Collection<Trade> = db.collection(models::trade::Trade::name().as_str());
	let filter_option = get_filter_option(params.order_by, params.desc).await;
	let query_find = params.query.to_doc();
	let mut cursor = col.find(query_find, filter_option).await?;
	let mut list_trade: Vec<TradeDTO> = Vec::new();
	while let Some(nft) = cursor.try_next().await? {
		list_trade.push(nft.into())
	}

	let total = get_total_page(list_trade.len(), params.size).await;
	Ok(Some(Page::<TradeDTO> {
		data: list_trade,
		message: EMPTY_STR.to_string(),
		page: params.page,
		size: params.size,
		total,
	}))
}
