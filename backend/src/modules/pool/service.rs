use futures_util::TryStreamExt;
use mongodb::{bson::doc, Collection, Database};
use shared::{constant::EMPTY_STR, models, BaseDocument, Pool};

use crate::common::{DBQuery, Page, QueryPage};

use super::dto::{PoolDTO, QueryFindPool};

pub async fn find_pool_by_query(
	params: QueryPage<QueryFindPool>,
	db: Database,
) -> shared::Result<Option<Page<PoolDTO>>> {
	let col: Collection<Pool> = db.collection(models::pool::Pool::name().as_str());
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
	let mut list_pools: Vec<PoolDTO> = Vec::new();
	let document = cursor.try_next().await?.ok_or("cursor try_next failed")?;
	let paginated_result = document.get_array("paginatedResults")?;
	paginated_result.into_iter().for_each(|rs| {
		let pool_str = serde_json::to_string(&rs).expect("Failed Parse game to String");
		let pool: Pool = serde_json::from_str(&pool_str).expect("Failed to Parse to NFT game");
		list_pools.push(pool.into());
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
	Ok(Some(Page::<PoolDTO> {
		message: EMPTY_STR.to_string(),
		data: list_pools,
		page: params.page,
		size: params.size,
		total: count as u64,
	}))
}
