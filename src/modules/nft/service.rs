use actix_web::Result;

use mongodb::{
    bson::{doc, extjson::de::Error},
    Collection, Database,
};

use crate::models::nft::NFT;
use crate::models::{self, nft_owner::NFTOwner};

use super::dto::NFTDTO;

pub async fn get_nft_by_token(token_id: &String, db: Database) -> Result<Option<NFTDTO>, Error> {
    let col: Collection<NFT> = db.collection(models::nft::NAME);
    let filter = doc! {"token_id": token_id};
    if let Ok(Some(nft_detail)) = col.find_one(filter, None).await {
        Ok(Some(nft_detail.into()))
    } else {
        Ok(None)
    }
}

/***
 * Get NFT by address of account
 */
pub async fn find_list_nft_by_address(
    address: &String,
    db: Database,
) -> Result<Option<Vec<NFTDTO>>, Error> {
    let filter = doc! {"address":address};
    let col: Collection<NFTOwner> = db.collection(models::nft_owner::NAME);
    let filter = doc! {"address":address};
    let mut cursor = col.find(filter, None).await;
    let mut list_nfts: Vec<NFTDTO> = Vec::new();
    Ok(Some(list_nfts))
}
