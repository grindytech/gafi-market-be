use mongodb::{Client, Database};

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
pub async fn init_db(_db: Database) {
	//TODO create indexes
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
