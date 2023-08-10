use crate::models;
use mongodb::{bson::doc, options::IndexOptions, Client, Database, IndexModel};

/**
 * The Client struct is the main entry point for the driver
 */
pub async fn get_client(uri: String) -> Client {
    let client = Client::with_uri_str(uri).await.expect("failed to connect");
    client
}
pub async fn get_database(uri: String, database_name: String) -> Database {
    let client = get_client(uri).await;
    let database = client.database(&database_name);
    database
}
async fn create_collection(client: &Client, db_name: &str, coll_name: &str) {
    let db = client.database(db_name);
    db.create_collection(coll_name, None).await.unwrap();
}
/**
 * init data code here: create index, init data,...
 */
pub async fn init_db(db: Database) {
    /*  let options = IndexOptions::builder().unique(true).build();
    let model = IndexModel::builder()
        .keys(doc! { "username": 1 })
        .options(options)
        .build();
    db.collection::<models::account::Account>(models::account::NAME)
        .create_index(model, None)
        .await
        .expect("creating an index should succeed"); */
}
