use futures_util::TryStreamExt;
use mongodb::{bson::doc, error, Collection, Database};
use shared::{bundle::Bundle, constant::EMPTY_STR, models};

use crate::common::{
	utils::{get_filter_option, get_total_page},
	DBQuery, ErrorResponse, Page, QueryPage,
};

use super::dto::{BundleDTO, QueryFindBundles};

//Find Bundle Detail By id
pub async fn find_bundle_by_id(
	bundle_id: &String,
	db: Database,
) -> Result<Option<BundleDTO>, error::Error> {
	let col: Collection<Bundle> = db.collection(models::bundle::NAME);

	let filter = doc! {"bundle_id":bundle_id};

	//Catch Error
	let bundle_detail = col.find_one(filter, None).await;
	match bundle_detail {
		Ok(Some(bundle_detail_doc)) => Ok(Some(bundle_detail_doc.into())),
		Ok(None) => Ok(None),
		Err(e) => Err(e),
	}
}
pub async fn find_bundles_by_query(
	params: QueryPage<QueryFindBundles>,
	db: Database,
) -> Result<Option<Page<BundleDTO>>, mongodb::error::Error> {
	let col: Collection<Bundle> = db.collection(models::bundle::NAME);

	let query_find = params.query.to_doc();
	let filter_option = get_filter_option(params.order_by, params.desc).await;
	let mut cursor = col.find(query_find, filter_option).await?;
	let mut list_games: Vec<BundleDTO> = Vec::new();
	while let Some(game) = cursor.try_next().await? {
		list_games.push(game.into())
	}

	let total = get_total_page(list_games.len(), params.size).await;
	Ok(Some(Page::<BundleDTO> {
		data: list_games,
		message: EMPTY_STR.to_string(),
		page: params.page,
		size: params.size,
		total,
	}))
}
