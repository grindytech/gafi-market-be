use mongodb::{bson::doc, options::IndexOptions, Client, Database, IndexModel};

use crate::{Account, BaseDocument, Block};

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

/**
 * init data code here: create index, init data,...
 */
pub async fn init_db(db: Database) {
	//Account index
	let options = IndexOptions::builder().unique(true).build();
	let model = IndexModel::builder().keys(doc! { "address": 1 }).options(options).build();
	db.collection::<Account>(&Account::name())
		.create_index(model, None)
		.await
		.expect("creating Account index should succeed");

	//BLock index
	let options = IndexOptions::builder().unique(true).build();
	let model = IndexModel::builder()
		.keys(doc! { "height": 1 ,"hash": 1})
		.options(options)
		.build();
	db.collection::<Block>("nft_block")
		.create_index(model.clone(), None)
		.await
		.expect("creating Block index should succeed");
	db.collection::<Block>("other_block")
		.create_index(model.clone(), None)
		.await
		.expect("creating Block index should succeed");

	//TODO db indexes
}
