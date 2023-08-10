use mongodb::Database;

use crate::utils::config::Config;

pub struct AppState {
    pub db: Database,
    pub config: Config,
}
