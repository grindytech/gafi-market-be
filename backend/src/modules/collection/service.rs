use std::collections::HashMap;

use actix_web::web::Query;
use futures_util::TryStreamExt;
use mongodb::{
	bson::{doc, Document},
	Collection, Database,
};
use serde_json::Value;
use shared::{models, models::nft_collection::NFTCollection, BaseDocument};

use crate::common::{
	utils::{get_filter_option, get_total_page},
	DBQuery, Page, QueryPage,
};

use super::dto::{NFTCollectionDTO, QueryFindCollections};
use shared::constant::EMPTY_STR;

//Find Collection Detail By ID
pub async fn find_collection_by_id(
	collection_id: &String,
	db: Database,
) -> Result<Option<NFTCollectionDTO>, mongodb::error::Error> {
	let col: Collection<NFTCollection> = db.collection(models::NFTCollection::name().as_str());
	let filter = doc! {"collection_id":collection_id};
	if let Ok(Some(collection_detail)) = col.find_one(filter, None).await {
		Ok(Some(collection_detail.into()))
	} else {
		Ok(None)
	}
}
pub async fn find_collections_by_query(
	params: QueryPage<QueryFindCollections>,
	db: Database,
) -> Result<Option<Page<NFTCollectionDTO>>, mongodb::error::Error> {
	let col: Collection<NFTCollection> = db.collection(models::nft_collection::NFTCollection::name().as_str());

	let query_find = params.query.to_doc();
	let filter_option = get_filter_option(params.order_by, params.desc).await;
	let mut cursor = col.find(query_find, filter_option).await?;
	let mut collections: Vec<NFTCollectionDTO> = Vec::new();
	while let Some(nft) = cursor.try_next().await? {
		collections.push(nft.into())
	}
	let total = get_total_page(collections.len(), params.size).await;
	Ok(Some(Page::<NFTCollectionDTO> {
		data: collections,
		message: EMPTY_STR.to_string(),
		page: params.page,
		size: params.size,
		total,
	}))
}

// Find Collections
