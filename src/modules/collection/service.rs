use mongodb::bson::doc;
use mongodb::{Collection, Database};

use super::dto::NFTCollectionDTO;
use crate::models::nft_collection::{self, NFTCollection};
use crate::models::{self};

pub async fn get_nft_collection_dto(
    collection_id: &String,
    db: Database,
) -> Result<Option<NFTCollectionDTO>, mongodb::error::Error> {
    let col: Collection<NFTCollection> = db.collection(models::nft_collection::NAME);
    let filter = doc! {"collection_id":collection_id};
    if let Ok(Some(collection_detail)) = col.find_one(filter, None).await {
        Ok(Some(collection_detail.into()))
    } else {
        eprint!("???");
        Ok(None)
    }
}
