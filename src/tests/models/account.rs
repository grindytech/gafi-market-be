use actix_web::test;
use dotenv::dotenv;
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::IndexOptions,
    Collection,
};
use mongodb::{error::Error, Client, IndexModel};

use crate::{
    config::Config,
    db,
    models::{
        self,
        account::{self, Account, SocialInfo},
    },
};
#[actix_web::test]
async fn test() -> Result<(), Error> {
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
    //Check Existing of address Test
    let query = doc! { "address": account_test2.address.clone() };
    let existing_document = col.find_one(query, None).await;
    if existing_document.is_err() {
        col.insert_one(account_test2, None)
            .await
            .expect("Failed Create new Account");
        Ok(())
    } else {
        Err(Error::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Unique constraint violation",
        )))
    }
}
