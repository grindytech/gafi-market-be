use crate::{
    models::{self, nft_collection::NFTCollection},
    utils::{config::Config, db},
};
use actix_web::test;
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
    let col: Collection<NFTCollection> = database.collection(models::nft_collection::NAME);
    let collection_test = NFTCollection {
        id: Some(ObjectId::new()),
        game_id: "Q29sbGVjdGlvblR5cGU6MTIxOTQ3Mg".to_string(),
        collection_id:"Q29sbGVjdGlvblR5cGU6MjQxOTc3MTc".to_string(),
        name: "Nakamigos".to_string(),
        slug: "nakamigos".to_string(),
        category:"Game 3d".to_string(),
        banner_url:Some("https://i.seadn.io/gae/wcmVaEjoHP8e1jU1YuDfeWtf10pR9erOoEiY8KupBuJhRoMbIn2OYepVY7bNU41WRLjwN8vobO-kXHKQ0LWtyVb6x_eh0Sv4PwF3Nw?auto=format&dpr=1&w=1920".to_string()),
        logo_url:Some("https://i.seadn.io/gae/de-K_S9pmwehlS5r4X-OLRSYY00L_rvqpPqNBfhK1KKfgIj-WKfHgaXod_tnXgc8iud4HoosANrb0k_TJMFySTcVsRaCyRtVp9ShDg?auto=format&dpr=1&w=256".to_string()),
        minting_fee:"300".to_string(),
        is_verified:false,
        update_at:1234567890,
        create_at:1234567231,
        raw:"No Data".to_string(),

    };
    col.insert_one(collection_test, None)
        .await
        .expect("Create New Collection");
}
