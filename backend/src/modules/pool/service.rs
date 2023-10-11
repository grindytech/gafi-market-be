use futures_util::TryStreamExt;
use mongodb::{Collection, Database};
use shared::{constant::EMPTY_STR, models, BaseDocument, Pool};

use crate::common::{utils::get_total_page, DBQuery, Page, QueryPage};

use super::dto::{PoolDTO, QueryFindPool};

pub async fn find_pool_by_query(
	params: QueryPage<QueryFindPool>,
	db: Database,
) -> Result<Option<Page<PoolDTO>>, mongodb::error::Error> {
	let col: Collection<Pool> = db.collection(models::pool::Pool::name().as_str());
	let filter = params.query.to_doc();

	let filter_option = mongodb::options::FindOptions::builder().sort(params.sort()).build();

	let mut cursor = col.find(filter, filter_option).await?;

	let mut list_pool: Vec<PoolDTO> = Vec::new();
	while let Some(pool) = cursor.try_next().await? {
		list_pool.push(pool.into())
	}
	let total = get_total_page(list_pool.len(), params.size).await;
	Ok(Some(Page::<PoolDTO> {
		message: EMPTY_STR.to_string(),
		data: list_pool,
		page: params.page,
		size: params.size,
		total,
	}))
}
