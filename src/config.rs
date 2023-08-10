#[derive(Debug, Clone)]

pub struct Config {
    pub mongodb_uri: String,
    pub mongodb_db_name: String,
    // pub jwt_secret: String,
    // pub jwt_max_age: i32,
}

impl Config {
    pub fn init() -> Config {
        let mongodb_uri = std::env::var("MONGODB_URI").expect("MONGODB_URI must be set");
        let mongodb_db_name =
            std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set");
        // let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        // let jwt_max_age = std::env::var("JWT_MAX_AGE").expect("JWT_MAX_AGE must be set");
        Config {
            mongodb_uri,
            mongodb_db_name,
            // jwt_secret,
            // jwt_max_age: jwt_max_age.parse::<i32>().unwrap(),
        }
    }
}
