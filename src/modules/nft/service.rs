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
        let nft_dto = NftDTO {
            token_id: nft_detail.token_id,
            collection_id: nft_detail.collection_id,
            name: nft_detail.name,
            amount: nft_detail.amount,
            is_burn: nft_detail.is_burn,
            description: nft_detail.description,
            status: nft_detail.status,
            external_url: nft_detail.external_url,
            weight: nft_detail.weight,
            img_url: nft_detail.img_url,
            visitor_count: nft_detail.visitor_count,
            favorite_count: nft_detail.favorite_count,
            propertise: nft_detail.propertise,
        };
        Ok(Some(nft_dto))
    } else {
        Ok(None)
    }
}
