use futures_util::TryStreamExt;
use mongodb::{bson::doc, Collection, Database};

use shared::{models, models::nft_collection::NFTCollection, BaseDocument};

use crate::common::{DBQuery, Page, QueryCollection};

use super::dto::NFTCollectionDTO;
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
