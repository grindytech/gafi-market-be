use crate::{config::Config, db};
use dotenv::from_filename;
use mongodb::Database;
use std::process::{Child, Command};

pub fn get_config() -> Config {
	from_filename(".env.test").ok();
	Config::init()
}

pub async fn get_database() -> Database {
	let config = get_config();
	db::get_database(config.mongodb_uri, config.mongodb_db_name).await
}

static mut IN_MEMORY_DB_PORT: u16 = 50001;
pub fn get_test_db() -> Child {
	unsafe { IN_MEMORY_DB_PORT += 1 };
	let cmd = Command::new("node")
    .arg("mongodb-memory.js")
		.arg(unsafe { IN_MEMORY_DB_PORT.to_string() })
		.spawn()
		.expect("mongodb-memory.js command failed to start");
	cmd
}

#[tokio::test]
pub async fn test_mongo_memory() {
	let mut db_process = get_test_db();
	let db = get_database().await;
	println!("{:?}", db.list_collection_names(None).await);
	let _ = db_process.kill();
}
