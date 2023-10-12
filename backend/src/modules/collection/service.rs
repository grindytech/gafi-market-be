use futures_util::TryStreamExt;
use mongodb::{bson::doc, Collection, Database};

use shared::{models, models::nft_collection::NFTCollection, BaseDocument};

use crate::common::{utils::get_total_page, DBQuery, Page, QueryPage};

use super::dto::{NFTCollectionDTO, QueryFindCollections};
use shared::constant::EMPTY_STR;

//Find Collection Detail By ID
pub async fn find_collection_by_id(
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

pub async fn find_collections(
	params: QueryPage<QueryFindCollections>,
	db: Database,
) -> Result<Option<Page<NFTCollectionDTO>>, mongodb::error::Error> {
	let col: Collection<NFTCollection> =
		db.collection(models::nft_collection::NFTCollection::name().as_str());

	let query_find = params.query.to_doc();

	let filter_option = mongodb::options::FindOptions::builder().sort(params.sort()).build();

	let mut cursor = col.find(query_find, filter_option).await?;
	let mut list_collections: Vec<NFTCollectionDTO> = Vec::new();

	while let Some(nft_collect) = cursor.try_next().await? {
		list_collections.push(nft_collect.into());
	}

	let total = get_total_page(list_collections.len(), params.size).await;
	Ok(Some(Page::<NFTCollectionDTO> {
		data: list_collections,
		message: EMPTY_STR.to_string(),
		page: params.page,
		size: params.size,
		total,
	}))
}
