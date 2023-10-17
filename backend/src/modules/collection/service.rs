use futures_util::TryStreamExt;
use mongodb::{bson::doc, Collection, Database};

use shared::{models, models::nft_collection::NFTCollection, BaseDocument, HistoryTx, NFTOwner};

use crate::common::{DBQuery, Page, QueryCollection};

use super::dto::{NFTCollectionDTO, NFTCollectionSupplyDTO, NFTCollectionVolumeDTO};
use shared::constant::EMPTY_STR;

pub async fn find_collections(
	params: QueryCollection,
	db: Database,
) -> shared::Result<Option<Page<NFTCollectionDTO>>> {
	let col: Collection<NFTCollection> =
		db.collection(models::nft_collection::NFTCollection::name().as_str());

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
	let mut list_collections: Vec<NFTCollectionDTO> = Vec::new();
	let document = cursor.try_next().await?.ok_or("cursor try_next failed")?;
	let paginated_result = document.get_array("paginatedResults")?;

	paginated_result.into_iter().for_each(|rs| {
		let collection_str = serde_json::to_string(&rs).expect("Failed Parse Collection to String");
		let collection: NFTCollection =
			serde_json::from_str(&collection_str).expect("Failed to Parse to NFT Collection");
		list_collections.push(collection.into());
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
	Ok(Some(Page::<NFTCollectionDTO> {
		data: list_collections,
		message: EMPTY_STR.to_string(),
		page: params.page,
		size: params.size,
		total: count as u64,
	}))
}

pub async fn find_collection_by_id(
	collection_id: String,
	db: Database,
) -> Result<Option<NFTCollection>, mongodb::error::Error> {
	let col: Collection<NFTCollection> =
		db.collection(models::nft_collection::NFTCollection::name().as_str());
	col.find_one(doc! {"collection_id":collection_id}, None).await
}
pub async fn find_collection_volume_data(
	collection_id: String,
	db: Database,
) -> shared::Result<Option<NFTCollectionVolumeDTO>> {
	let col: Collection<HistoryTx> = db.collection(models::history_tx::HistoryTx::name().as_str());

	let filter = doc! {
		"$match":{
			"nfts.collection":collection_id.parse::<i32>()?,
			"event":shared::constant::EVENT_MINTED
		}
	};
	let group = doc! {
		"$group": {
			"_id": "$nfts.collection",
			"min_price": {
				"$min": "$price"
			},
			"max_price": {
				"$max": "$price"
			},
			"volume":{
				"$sum":"$value"
			},
			"sold": {
				"$sum": "$amount"
			},
		}
	};
	let options = mongodb::options::AggregateOptions::builder().allow_disk_use(true).build();
	let mut cursor = col.aggregate(vec![filter, group], options).await?;

	if let Some(doc) = cursor.try_next().await? {
		let collection_analysis = NFTCollectionVolumeDTO::convert_document_to_dto(doc)?;
		return Ok(Some(collection_analysis));
	}

	return Ok(None);
}

pub async fn find_collection_supply_data(
	collection_id: String,
	db: Database,
) -> shared::Result<Option<NFTCollectionSupplyDTO>> {
	let col: Collection<NFTOwner> = db.collection(models::nft_owner::NFTOwner::name().as_str());

	let filter = doc! {
		"$match":{
			"collection_id":collection_id,
		}
	};
	let group = doc! {
		"$group": {
			"_id": "$collection_id",
			"owner": { "$sum": 1 },
			"total_supply": { "$sum": "$amount" }
		}
	};

	let mut cursor = col.aggregate(vec![filter, group], None).await?;

	if let Some(doc) = cursor.try_next().await? {
		let col_analysis = NFTCollectionSupplyDTO::convert_document_to_dto(doc)?;
		return Ok(Some(col_analysis));
	};
	Ok(None)
}
