use actix_web::test;
use dotenv::dotenv;
use mongodb::{bson::oid::ObjectId, Collection};

use crate::{
    config::{self, Config},
    db,
    models::{
        self,
        nft::{self, Propertise, NFT},
    },
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
    let col: Collection<NFT> = database.collection(models::nft::NAME);
    let propertise_Test: Vec<Propertise> = vec![
        Propertise {
            key: "Color".to_string(),
            value: "Shiny Red".to_string(),
        },
        Propertise {
            key: "Light".to_string(),
            value: "Karas Red".to_string(),
        },
    ];
    let nft_test = NFT {
        id: Some(ObjectId::new()),
        collection_id:"Q29sbGVjdGlvblR5cGU6MjQxOTc3MTc".to_string(),
        token_id: "0xd774557b647330c91bf44cfeab205095f7e6c368".to_string(),
        is_burn:false,
        amount: 20,
        name:"#189337".to_string(),
        description:"HV-MTL is a dynamic NFT collection consisting of mechs summoned through a space-time rift that has opened up outside the Bored Ape Yacht Club. Every HV (pronounced: Heavy) starts as a Core. Once unlocked, each Core transforms into a one-of-a-kind mech designed to evolve in the right environment.".to_string(),
        status:"list".to_string(),
        external_url:"https://mdvmm.xyz/".to_string(),
        img_url:"https://i.seadn.io/gcs/files/56dcfd26412b7e61a55af3a63f1ce8ab.png?auto=format&dpr=1&w=1000".to_string(),
        weight:"15".to_string(),
        propertise:propertise_Test,
        visitor_count:10,
        favorite_count:20,
    };
    col.insert_one(nft_test, None)
        .await
        .expect("Create New NFT Success");
}
