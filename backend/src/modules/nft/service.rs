use actix_web::Result;

use futures_util::TryStreamExt;
use mongodb::{
	bson::{doc, Document},
	Collection, Database,
};

use crate::{
	common::{
		utils::{get_filter_option, get_total_page},
		DBQuery, Page, QueryPage,
	},
	shared::constant::EMPTY_STR,
};

use super::dto::{QueryFindNFts, NFTDTO};
use shared::{
	models::{self, nft::NFT, nft_owner::NFTOwner},
	BaseDocument,
};

pub async fn find_nft_by_token(
	token_id: &String,
	db: Database,
) -> Result<Option<NFTDTO>, mongodb::error::Error> {
	let col: Collection<NFT> = db.collection(models::nft::NFT::name().as_str());
	let filter = doc! {"token_id": token_id};
	if let Ok(Some(nft_detail)) = col.find_one(filter, None).await {
		/* log::info!("NFT Detail {:?}", nft_detail); */
		Ok(Some(nft_detail.into()))
	} else {
		Ok(None)
	}
}

pub async fn find_nfts_by_address(
	params: QueryPage<QueryFindNFts>,
	db: Database,
) -> Result<Option<Page<NFTDTO>>, mongodb::error::Error> {
	let col: Collection<NFTOwner> = db.collection(models::nft_owner::NFTOwner::name().as_str());
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
	let col_nft: Collection<NFT> = db.collection(models::nft::NFT::name().as_str());
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

pub async fn find_nfts_by_query(
	params: QueryPage<QueryFindNFts>,
	db: Database,
) -> Result<Option<Page<NFTDTO>>, mongodb::error::Error> {
	let col: Collection<NFT> = db.collection(models::nft::NFT::name().as_str());
	let filter_option = get_filter_option(params.order_by, params.desc).await;
	let query_find = params.query.to_doc();
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
