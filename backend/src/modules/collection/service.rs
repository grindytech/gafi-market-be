use futures_util::{StreamExt, TryStreamExt};
use mongodb::{
	bson::{doc, Document},
	Collection, Database,
};

use shared::{models, models::nft_collection::NFTCollection, BaseDocument, HistoryTx, NFTOwner};

use crate::common::{DBQuery, Page, QueryCollection};

use super::dto::{NFTCollectionDTO, NFTCollectionSupplyData, NFTCollectionVolumeDTO};
use shared::constant::EMPTY_STR;

//Find Collection Detail By ID
/* pub async fn find_collection_by_id(
	collection_id: &String,
	db: Database,
) -> shared::Result<Option<NFTCollectionDTO>> {
	let col: Collection<NFTCollection> = db.collection(models::NFTCollection::name().as_str());
	let filter = doc! {
		"$match":{
			"collection_id":collection_id
		}
	};
	let filter_lookup = doc! {
	"$lookup":{
		"from":"nft",
		"localField":"collection_id", "foreignField":"collection_id",
		"as": "nfts"
		}
	};
	let mut cursor = col.aggregate(vec![filter, filter_lookup], None).await?;

	if let Some(doc) = cursor.try_next().await? {
		let collection_str = serde_json::to_string(&doc).expect("fail to parse string");

		let collection_detail: NFTCollection =
			serde_json::from_str(&collection_str).expect("fail to parse NFT Collection");

		Ok(Some(collection_detail.into()))
	} else {
		// No matching document found, return None
		Ok(None)
	}
}
 */
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
) -> shared::Result<NFTCollectionSupplyData> {
	let col: Collection<NFTOwner> = db.collection(models::nft_owner::NFTOwner::name().as_str());

	let filter = doc! {
		"$match":{
			"collection_id":collection_id.parse::<i32>()?,
		}
	};
	let group = doc! {
		"$group": {
			"_id": "$collection_id",
			"owner":{"$size":"$owners"},
			"total_supply":{"$sum":"$owners.amount"}
		}
	};

	let options = mongodb::options::AggregateOptions::builder().allow_disk_use(true).build();
	let mut colelction_analysis = NFTCollectionSupplyData {
		total_supply: 0,
		owner: 0,
	};
	let mut cursor = col.aggregate(vec![filter, group], options).await?;

	if let Some(doc) = cursor.try_next().await? {
		log::info!("What doc {:?}", doc);
		let collection_str = serde_json::to_string(&doc).expect("fail to parse string");

		let colelction_analysis: NFTCollectionSupplyData =
			serde_json::from_str(&collection_str).expect("Failed to parse Colelciton Analysis");
		return Ok(colelction_analysis);
	}
	Ok(colelction_analysis)
}
