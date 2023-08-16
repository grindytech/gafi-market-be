use crate::{
    config::Config,
    db,
    models::{self, auction::Auction},
};
use actix_web::test;
use chrono::Utc;
use dotenv::dotenv;
use mongodb::{
    bson::{doc, oid::ObjectId},
    Collection,
};
#[actix_web::test]
async fn test() {
    dotenv().ok();
    let configuration = Config::init();
    let database = db::get_database(
        configuration.mongodb_uri.clone(),
        configuration.mongodb_db_name.clone(),
    )
    .await;
    let col: Collection<Auction> = database.collection(models::auction::NAME);
    let auction_test = Auction {
        id: Some(ObjectId::new()),
        auction_id: "0x86c10d10eca1fca9daf87a279abccabe0063f247".to_string(),
        token_id: "".to_string(),
        creator: "0sxbdfc529688922fb5036d9439a7cd61d61114f600".to_string(),
        method: "".to_string(),
        status: "LISTED".to_string(),
        reserve_price: 3333,
        start_price: 3333,
        end_price: 3333,
        begin_at: Utc::now().timestamp_millis(),
        update_at: Utc::now().timestamp_millis(),
        create_at: Utc::now().timestamp_millis(),
        end_at: Utc::now().timestamp_millis(),
    };
    col.insert_one(auction_test, None).await.expect("");
}
