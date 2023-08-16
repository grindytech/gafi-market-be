use crate::{
    config::Config,
    db,
    models::{self, sale::Sale},
};
use actix_web::test;
use chrono::Utc;
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
    let col: Collection<Sale> = database.collection(models::sale::NAME);
    let sale_test = Sale {
        id: Some(ObjectId::new()),
        sale_id: "8x89974557b647330c91bf44cfeab205095f7e6c367".to_string(),
        token_id: "0xd774557b647330c91bf44cfeab205095f7e6c367".to_string(),
        quantity: 10,
        creator: "0sxbdfc529688922fb5036d9439a7cd61d61114f600".to_string(),
        type_sale: crate::models::sale::TypeSale::FixPrice(true),
        method: "LIST_SALE".to_string(),
        list_price: 30,
        begin_at: Utc::now().timestamp_millis(),
        update_at: Utc::now().timestamp_millis(),
        create_at: Utc::now().timestamp_millis(),
        end_at: Utc::now().timestamp_millis(),
    };
    col.insert_one(sale_test, None)
        .await
        .expect("Failed Create Sale Test");
}
