use std::collections::HashMap;

use actix_web::Result;

use futures_util::TryStreamExt;
use mongodb::{
	bson::{doc, Document},
	options::FindOptions,
	Collection, Database,
};

use crate::{
	common::{
		utils::{get_filter_option, get_total_page},
		Page, QueryPage,
	},
	shared::constant::EMPTY_STR,
};

use super::dto::{QueryFindNFts, NFTDTO};
use shared::models::{self, nft::NFT, nft_owner::NFTOwner};

pub async fn find_nft_by_token(
	token_id: &String,
	db: Database,
) -> Result<Option<NFTDTO>, mongodb::error::Error> {
	let col: Collection<NFT> = db.collection(models::nft::NAME);
	let filter = doc! {"token_id": token_id};
	if let Ok(Some(nft_detail)) = col.find_one(filter, None).await {
		/* log::info!("NFT Detail {:?}", nft_detail); */
		Ok(Some(nft_detail.into()))
	} else {
		Ok(None)
	}
}
/* pub async fn find_nfs_by_name(
	params: QueryPage<QueryFindNFts>,
	db: Database,
) -> Result<Option<Page<NFTDTO>>, mongodb::error::Error> {
	let col: Collection<NFT> = db.collection(models::nft::NAME);
	let filter = doc! {"name":params.query.name};

	let mut cursor = col.find(filter, None).await?;
	let mut list_nfts: Vec<NFTDTO> = Vec::new();
	while let Some(nft) = cursor.try_next().await? {
		list_nfts.push(nft.into())
	}
	let total = get_total_page(list_nfts.len(), params.size).await;
	Ok(Some(Page::<NFTDTO> {
		data: list_nfts,
		message: EMPTY_STR.to_string(),
		page: params.page,
		size: params.size,
		total,
	}))
} */

/***
 * Get NFT by address of account
 */

pub async fn find_nfts_by_address(
	params: QueryPage<QueryFindNFts>,
	db: Database,
) -> Result<Option<Page<NFTDTO>>, mongodb::error::Error> {
	let col: Collection<NFTOwner> = db.collection(models::nft_owner::NAME);
	let address = params.query.address;
	let filter = doc! {"address": address};

	// Get List nftowner of this address
	let cursor = col.find(filter, None).await?;
	let list_nftowner: Vec<NFTOwner> = cursor.try_collect().await.unwrap();
	/* log::info!("DATA {:?}", list_nftowner); */
	let mut or_filters = Vec::new();

	for filter in &list_nftowner {
		or_filters.push(doc! {
			"$and": [
				{"token_id": &filter.token_id},
				{"collection_id": &filter.collection_id},
			]
		});
	}

	let query = doc! {
		"$or": or_filters
	};
	let filter_option = get_filter_option(params.order_by, params.desc).await;
	let col_nft: Collection<NFT> = db.collection(models::nft::NAME);
	let mut cursor_nft = col_nft.find(query, filter_option).await?;
	let mut list_nfts: Vec<NFTDTO> = Vec::new();

	while let Some(nft) = cursor_nft.try_next().await? {
		list_nfts.push(nft.into())
	}
	let total = get_total_page(list_nfts.len(), params.size).await;
	Ok(Some(Page::<NFTDTO> {
		data: list_nfts,
		message: EMPTY_STR.to_string(),
		page: params.page,
		size: params.size,
		total,
	}))
}

//TODO implement paging, order, find nfts by query, search by name
pub async fn find_nfts_by_query(
	params: QueryPage<QueryFindNFts>,
	db: Database,
) -> Result<Option<Page<NFTDTO>>, mongodb::error::Error> {
	let col: Collection<NFT> = db.collection(models::nft::NAME);
	let filter_option = get_filter_option(params.order_by, params.desc).await;

	let query_find = {
		let mut or_conditions = vec![];

		if let Some(token) = params.query.token_id.clone() {
			if !token.is_empty() {
				or_conditions.push(doc! {"token_id": token});
			}
		}

		if let Some(name) = params.query.name.clone() {
			if !name.is_empty() {
				or_conditions.push(doc! {"name": name});
			}
		}

		if let Some(collection_id) = params.query.collection_id.clone() {
			if !collection_id.is_empty() {
				or_conditions.push(doc! {"collection_id": collection_id});
			}
		}

		if !or_conditions.is_empty() {
			doc! {"$or": or_conditions}
		} else {
			Document::new()
		}
	};
	// create query optional => pass criteria
	/* let mut criteria = HashMap::new();
	criteria.insert("token_id".to_string(), params.query.token_id);
	criteria.insert("name".to_string(), params.query.name);
	criteria.insert("collection_id".to_string(), params.query.collection_id);

	let query_find = create_or_query(criteria).await; */
	let mut cursor_nft = col.find(query_find, filter_option).await?;

	let mut list_nfts: Vec<NFTDTO> = Vec::new();
	while let Some(nft) = cursor_nft.try_next().await? {
		list_nfts.push(nft.into())
	}

	let total = get_total_page(list_nfts.len(), params.size).await;
	Ok(Some(Page::<NFTDTO> {
		data: list_nfts,
		message: EMPTY_STR.to_string(),
		page: params.page,
		size: params.size,
		total,
	}))
}
