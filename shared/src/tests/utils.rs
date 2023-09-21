use crate::{config::Config, db, utils};
use dotenv::from_filename;
use mongodb::Database;
use std::{env, process::Stdio};
use subxt::utils::AccountId32;
use tokio::{
	io::{AsyncBufReadExt, BufReader},
	process::{Child, Command},
};

pub fn mock_account_id32() -> (AccountId32, String) {
	let public_key = "ec84321d9751c066fb923035073a73d467d44642c457915e7496c52f45db1f65";
	let account_u8 = utils::vec_to_array(hex::decode(public_key).unwrap());
	let account = AccountId32::from(account_u8);
	(account, public_key.to_string())
}

pub fn get_config() -> Config {
	from_filename(".env.test").ok();
	Config::init()
}

pub async fn get_database() -> Database {
	let config = get_config();
	db::get_database(config.mongodb_uri, config.mongodb_db_name).await
}

pub fn start_test_db(max_life_time: u32) -> Child {
	let mut path = env::current_exe().unwrap();
	path.pop(); // remove the binary name
	let full_path = path.to_string_lossy();
	let end_index = full_path.find("/target").unwrap_or(full_path.len());
	let full_path_js_file = format!(
		"{}/mongodb-memory.js",
		&full_path[0..end_index]
	);

	let cmd = Command::new("node")
		.arg(full_path_js_file)
		.arg(max_life_time.to_string())
		.stdout(Stdio::piped())
		.spawn()
		.expect("mongodb-memory.js command failed to start");
	cmd
}

pub async fn get_test_db(max_life_time: u32) -> (Child, Database) {
	let mut db_process = start_test_db(max_life_time);
	let stdout = db_process.stdout.take().expect("child did not have a handle to stdout");
	let mut stdout_reader = BufReader::new(stdout).lines();
	let db_connect_string = stdout_reader.next_line().await.unwrap().unwrap();
	let db = db::get_database(db_connect_string, "test".to_string()).await;
	(db_process, db)
}

#[tokio::test]
pub async fn test_mongo_memory() {
	let (mut db_process, db) = get_test_db(60_000).await;
	println!("{:?}", db.list_collection_names(None).await);
	let _ = db_process.kill();
}
