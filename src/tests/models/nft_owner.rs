use crate::{
    config::Config,
    db,
    models::{self, nft_owner::NFTOwner},
};
use actix_web::test;
use dotenv::dotenv;
use mongodb::{bson::oid::ObjectId, Collection};
#[actix_web::test]
async fn test() {
    dotenv().ok();
    let configuration = Config::init();
    let database = db::get_database(
        configuration.mongodb_uri.clone(),
        configuration.mongodb_db_name.clone(),
    )
    .await;
    // init collection
    let col: Collection<NFTOwner> = database.collection(models::nft_owner::NAME);
    let nft_owner_test = NFTOwner {
        id: Some(ObjectId::new()),
        token_id: "0xd774557b647330c91bf44cfeab205095f7e6c367".to_string(),
        collection_id: "Q29sbGVjdGlvblR5cGU6MjQxOTc3MTc".to_string(),
        address: "0sxbdfc529688922fb5036d9439a7cd61d61114f600".to_string(),
        lock: 20,
        amount: 50,
        create_at: 1234567231,
    };
    col.insert_one(nft_owner_test, None)
        .await
        .expect("Create NFT_Owner");
}
