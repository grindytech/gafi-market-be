use futures_util::TryStreamExt;
use mongodb::{bson::doc, Collection, Database};

use crate::{
	common::{DBQuery, NFTPage, Page, QueryNFT, QueryPage},
	shared::constant::EMPTY_STR,
};

use super::dto::{NFTOwnerOfDto, QueryFindNFts, NFTDTO};
use shared::{
	models::{self, nft::NFT, nft_owner::NFTOwner},
	BaseDocument,
};

pub async fn find_nfts_with_owner(
	params: QueryNFT,
	db: Database,
) -> shared::Result<Option<Page<NFTOwnerOfDto>>> {
	let col: Collection<NFTOwner> = db.collection(models::nft_owner::NFTOwner::name().as_str());
	let filter = params.query.to_doc();
	let filter_match = doc! {
		"$match": filter,
	};
	let filter_lookup = doc! {
		"$lookup": {
			"from": "nft",
			"let": {
				"nft_collection_id": "$collection_id",
				"nft_token_id": "$token_id"
			},
			"pipeline": [
				{
					"$match": {
						"$expr": {
							"$and": [
								{
									"$eq": [ "$collection_id", "$$nft_collection_id" ]
								},
								{
									"$eq": [ "$token_id", "$$nft_token_id" ]
								}
							]
						}
						//more NFT filter here
					},
				},
				{
					"$sort": params.sort(),
				},
			],
			"as": "nft",
		},
	};
	let paging = doc! {
	  "$facet":{
			"paginatedResults": [ { "$skip": params.skip() }, { "$limit": params.size() } ],
		  "totalCount": [ { "$count": "count" } ]
		},
	};

	let mut cursor = col.aggregate(vec![filter_match, filter_lookup, paging], None).await?;
	let mut list_nfts: Vec<NFTOwnerOfDto> = Vec::new();
	let document = cursor.try_next().await?.ok_or("cursor try_next failed")?;

	let paginated_results = document.get_array("paginatedResults")?;

	paginated_results.into_iter().for_each(|rs| {
		let nft_str = serde_json::to_string(&rs).expect("fail to parse string");
		let owner_nft: NFTOwner = serde_json::from_str(&nft_str).expect("fail to parse NFTOwner");
		list_nfts.push(owner_nft.into());
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

	Ok(Some(Page::<NFTOwnerOfDto> {
		data: list_nfts,
		message: EMPTY_STR.to_string(),
		page: params.page,
		size: params.size,
		total: count as u64,
	}))
}

pub async fn find_nfts_by_query(
	params: QueryPage<QueryFindNFts>,
	db: Database,
) -> shared::Result<Option<NFTPage>> {
	let col: Collection<NFT> = db.collection(models::nft::NFT::name().as_str());
	let query_find = params.query.to_doc();
	let filter_match = doc! {
		"$match":query_find,
	};
	let sort = doc! {
		"$sort":params.sort()
	};
	let paging = doc! {
	  "$facet":{
			"paginatedResults": [ { "$skip": params.skip() }, { "$limit": params.size() } ],
		  "totalCount": [ { "$count": "count" } ],
		},
	};

	let mut cursor = col.aggregate(vec![filter_match, sort, paging], None).await?;

	let document = cursor.try_next().await?.ok_or("cursor try_next failed")?;
	let paginated_result = document.get_array("paginatedResults")?;
	let mut list_nfts: Vec<NFTDTO> = Vec::new();
	paginated_result.into_iter().for_each(|rs| {
		let nft_str = serde_json::to_string(&rs).expect("Failed to Parase NFT to String");
		let nft: NFT = serde_json::from_str(&nft_str).expect("Failed Parse to NFT");
		list_nfts.push(nft.into());
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
	Ok(Some(NFTPage {
		data: list_nfts,
		message: EMPTY_STR.to_string(),
		page: params.page,
		size: params.size,
		total: count as u64,
	}))
}
