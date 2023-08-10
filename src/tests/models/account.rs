use actix_web::test;
use dotenv::dotenv;
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::IndexOptions,
    Collection,
};
use mongodb::{Client, IndexModel};

use crate::{
    models::{
        self,
        account::{self, Account, SocialInfo},
    },
    utils::{config::Config, db},
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
    let col: Collection<Account> = database.collection(models::account::NAME);
    let social_objects = SocialInfo {
        twitter: Some("karas".to_string()),
        web: None,
        medium: None,
        facebook: None,
        discord: None,
    };
    let new_object_id = ObjectId::new();

    let account_test2 = Account {
        id: Some(new_object_id),
        address: "0x156fb9dB6Cb952DaAebBF080974022271988868F".to_string(),
        balance: "200.00".to_string(),
        is_verified: true,
        name: "HYTOPIA-Controller".to_string(),
        bio: "Controlling wallet used by the official TOPIA team..".to_string(),
        social: social_objects,
        logo_url: Some("https://i.seadn.io/gcs/files/fba7a97bc551230dadbfd8f997da278b.jpg?auto=format&dpr=1&w=384".to_string()),
        banner_url: None,
        update_at: 1234567890,
        create_at: 1234567890,
    };
    col.insert_one(account_test2, None)
        .await
        .expect("Create new Account");
}
