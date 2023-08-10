use crate::utils::{config::Config, db};
use dotenv::from_filename;
use mongodb::Database;
pub fn get_config() -> Config {
    from_filename(".env").ok();
    Config::init()
}

pub async fn get_database() -> Database {
    let config = get_config();
    db::get_database(config.mongodb_uri, config.mongodb_db_name).await
}

#[test]

fn test() {
    let config = get_config();
    // Test get data from dot env file
    assert_eq!(config.mongodb_db_name.as_str(), "market-gafi");
}
