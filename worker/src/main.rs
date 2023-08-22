use dotenv::dotenv;
use env_logger::Env;
use shared::{db, types::Result, Config};
use workers::Worker;

// Generate an interface that we can use from the node's metadata.
#[subxt::subxt(runtime_metadata_path = "./metadata.scale")]
pub mod gafi {}

mod tasks;
mod workers;

#[tokio::main]
async fn main() -> Result<()> {
	dotenv().ok();

	let configuration = Config::init();
	let database = db::get_database(
		configuration.mongodb_uri.clone(),
		configuration.mongodb_db_name.clone(),
	)
	.await;
	db::init_db(database.clone()).await;
	env_logger::init_from_env(Env::default().default_filter_or("info"));

	let mut worker = Worker::new(database, None, Some(127200), None, None).await?;
	let tasks = tasks::create_tasks();
	for task in tasks {
		worker.register(task);
	}

	worker.start(1000).await?;

	Ok(())
}
