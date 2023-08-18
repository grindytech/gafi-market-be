use mongodb::Database;
use shared::Config;

pub struct AppState {
	pub db: Database,
	pub config: Config,
}
