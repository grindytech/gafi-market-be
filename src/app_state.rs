use crate::config::Config;
use mongodb::Database;

pub struct AppState {
    pub db: Database,
    pub config: Config,
}
