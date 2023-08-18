use crate::{config::Config, db};
use dotenv::from_filename;
use mongodb::Database;

pub fn get_config() -> Config {
	from_filename(".env.test").ok();
	Config::init()
}

pub async fn get_database() -> Database {
	let config = get_config();
	db::get_database(config.mongodb_uri, config.mongodb_db_name).await
}
