use actix_web::Result;

use mongodb::{
    bson::{doc, extjson::de::Error},
    Collection, Database,
};

use crate::models;
use crate::models::nft::NFT;

use super::dto::{NftDTO, PropertiseDTO};

pub async fn get_nft(token_id: &String, db: Database) -> Result<Option<NftDTO>, Error> {
    let col: Collection<NFT> = db.collection(models::nft::NAME);
    let filter = doc! {"token_id": token_id};
    if let Ok(Some(nft_detail)) = col.find_one(filter, None).await {
        Ok(Some(nft_detail.into()))
    } else {
        Ok(None)
    }
}
