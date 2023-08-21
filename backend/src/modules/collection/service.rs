use actix_web::web::Query;
use mongodb::{bson::doc, Collection, Database};

use super::dto::NFTCollectionDTO;
use shared::{models, models::nft_collection::NFTCollection};

//Find Collection Detail By ID
pub async fn find_collection_by_id(
	collection_id: &String,
	db: Database,
) -> Result<Option<NFTCollectionDTO>, mongodb::error::Error> {
	let col: Collection<NFTCollection> = db.collection(models::nft_collection::NAME);
	let filter = doc! {"collection_id":collection_id};
	if let Ok(Some(collection_detail)) = col.find_one(filter, None).await {
		Ok(Some(collection_detail.into()))
	} else {
		Ok(None)
	}
}

// Find Collections
