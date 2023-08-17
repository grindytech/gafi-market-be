use actix_web::Result;

use futures_util::TryStreamExt;
use mongodb::{
    bson::{doc, extjson::de::Error, Bson},
    Collection, Database,
};

use crate::models::nft::NFT;
use crate::models::{self, nft_owner::NFTOwner};

use super::dto::NFTDTO;

pub async fn get_nft_by_token(token_id: &String, db: Database) -> Result<Option<NFTDTO>, Error> {
    let col: Collection<NFT> = db.collection(models::nft::NAME);
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
pub async fn find_list_nft_by_address(
    address: &String,
    db: Database,
) -> Result<Option<Vec<NFTDTO>>, mongodb::error::Error> {
    let col: Collection<NFTOwner> = db.collection(models::nft_owner::NAME);
    let filter = doc! {"address":address};

    // Get List nftowner of this address
    let cursor = col.find(filter, None).await?;
    let list_nftowner: Vec<NFTOwner> = cursor.try_collect().await.unwrap();
    /*   log::info!("DATA {:?}", list_nftowner); */
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

    let col_nft: Collection<NFT> = db.collection(models::nft::NAME);
    let mut cursor_nft = col_nft.find(query, None).await?;
    let mut list_nfts: Vec<NFTDTO> = Vec::new();

    while let Some(nft) = cursor_nft.try_next().await? {
        list_nfts.push(nft.into())
    }
    log::info!("{:?}", list_nfts);
    Ok(Some(list_nfts))
}
