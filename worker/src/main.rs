use dotenv::dotenv;
use env_logger::Env;
use mongodb::Database;
use shared::{db, types::Result, Config};
use workers::Worker;

// Generate an interface that we can use from the node's metadata.
#[subxt::subxt(runtime_metadata_path = "./chain-metadata/v14-102r.scale")]
pub mod gafi {}

mod event_handler;
mod services;
#[cfg(test)]
mod tests;
mod types;
mod workers;

async fn get_db() -> Database {
	let configuration = Config::init();
	let database = db::get_database(
		configuration.mongodb_uri.clone(),
		configuration.mongodb_db_name.clone(),
	)
	.await;
	database
}

#[tokio::main]
async fn main() -> Result<()> {
	dotenv().ok();
	env_logger::init_from_env(Env::default().default_filter_or("debug"));
	let database = get_db().await;
	db::init_db(database.clone()).await;

	//worker process nft event
	let run_worker_1 = async {
		let configuration = Config::init();
		let database = get_db().await;
		let mut nft_worker = Worker::new(
			"nft".to_lowercase(),
			database.clone(),
			configuration.start_block,
			configuration.rpc,
			None,
		)
		.await
		.unwrap();

		nft_worker.add_tasks(&mut event_handler::nft::tasks());
		nft_worker.add_tasks(&mut event_handler::trade::retail::tasks());
		nft_worker.add_tasks(&mut event_handler::trade::bundle::tasks());
		nft_worker.add_tasks(&mut event_handler::trade::swap::tasks());
		nft_worker.add_tasks(&mut event_handler::trade::wishlist::tasks());
		nft_worker.add_tasks(&mut event_handler::trade::auction::tasks());
		nft_worker.add_tasks(&mut event_handler::trade::cancel_trade::tasks());

		let _ = nft_worker.start(1000).await;
	};

	//all other jobs
	let run_worker_2 = async {
		let configuration = Config::init();
		let database = get_db().await;
		let mut other_worker = Worker::new(
			"other".to_lowercase(),
			database.clone(),
			configuration.start_block,
			configuration.rpc,
			None,
		)
		.await
		.unwrap();
		let mut other_tasks = vec![];

		other_tasks.append(&mut event_handler::collection::tasks());
		other_tasks.append(&mut event_handler::pool::tasks());
		other_tasks.append(&mut event_handler::game::tasks());

		other_worker.add_tasks(&mut other_tasks);
		let _ = other_worker.start(1000).await;
	};

	let t1 = tokio::spawn(run_worker_1);
	let t2: tokio::task::JoinHandle<_> = tokio::spawn(run_worker_2);
	let (_, _) = (t1.await?, t2.await?);

	Ok(())
}
