use actix_web::Result;

use futures_util::TryStreamExt;
use mongodb::{
	bson::{doc, extjson::de::Error},
	Collection, Database,
};

use crate::{
	common::{Page, QueryPage},
	shared::constant::EMPTY_STR,
};

use super::dto::{QueryFindNFts, NFTDTO};
use shared::{models::{self, nft::NFT, nft_owner::NFTOwner}, BaseDocument};

pub async fn get_nft_by_token(token_id: &String, db: Database) -> Result<Option<NFTDTO>, Error> {
	let col: Collection<NFT> = db.collection(models::NFT::name().as_str());
	let filter = doc! {"token_id": token_id};
	if let Ok(Some(nft_detail)) = col.find_one(filter, None).await {
		log::info!("NFT Detail {:?}", nft_detail);
		Ok(Some(nft_detail.into()))
	} else {
		Ok(None)
	}
}

/***
 * Get NFT by address of account
 */
pub struct LetTestStruct {
	token_id: String,
	collection_id: String,
}

//TODO implement paging, order, find nfts by query, search by name
pub async fn find_nfts(
	params: QueryPage<QueryFindNFts>,
	db: Database,
) -> Result<Option<Page<NFTDTO>>, mongodb::error::Error> {
	let col: Collection<NFTOwner> = db.collection(models::NFTOwner::name().as_str());
	let address = params.query.address;
	let filter = doc! {"address": address};

	// Get List nftowner of this address
	let cursor = col.find(filter, None).await?;
	let list_nftowner: Vec<NFTOwner> = cursor.try_collect().await.unwrap();
	/* log::info!("DATA {:?}", list_nftowner); */
	let mut or_filters = Vec::new();
	log::info!("{:?}", list_nftowner);
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

	let col_nft: Collection<NFT> = db.collection(models::NFT::name().as_str());
	let mut cursor_nft = col_nft.find(query, None).await?;
	let mut list_nfts: Vec<NFTDTO> = Vec::new();

	while let Some(nft) = cursor_nft.try_next().await? {
		list_nfts.push(nft.into())
	}
	log::info!("{:?}", list_nfts);
	Ok(Some(Page::<NFTDTO> {
		data: list_nfts,
		message: EMPTY_STR.to_string(),
		page: params.page,
		size: params.size,
		total: 0, //TODO get total
	}))
}
