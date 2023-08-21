use std::sync::Arc;

use crate::gafi;

use super::*;
use log::logger;
use mongodb::{bson::doc, options::FindOneOptions, results::InsertOneResult, Collection, Database};
use shared::{block, types::Result, BaseDocument, Block};
use subxt::{events::EventDetails, OnlineClient, PolkadotConfig};
use tokio::{
	sync::{Mutex, MutexGuard},
	time::{sleep, Duration},
};

pub struct WorkerState {
	tasks: Vec<Box<Task>>,
	current_block: u32,
	latest_block: u32,
	db: Database,
	api: OnlineClient<PolkadotConfig>,
	finalize_delay: u32,
	rpc: String,
}
pub struct WorkerRunningStatus {
	enabled: Arc<bool>,
	running: Arc<bool>,
}
pub type WorkerStateSync = Arc<Mutex<WorkerState>>;

impl WorkerState {
	pub async fn new(
		db: Database,
		finalize_delay: Option<u32>,
		start_block: Option<u32>,
		rpc: Option<String>,
	) -> Result<Self> {
		let finalize_delay = finalize_delay.unwrap_or(3);
		let start_block = start_block.unwrap_or(0);
		let rpc = rpc.unwrap_or("wss://rpc-testnet.gafi.network:443".to_string());

		let api = OnlineClient::<PolkadotConfig>::from_url(&rpc).await?;

		let sort = doc! {"height": -1};
		let option = FindOneOptions::builder().sort(sort).build();
		let collection: Collection<block::Block> = db.collection(block::Block::name().as_str());
		let last_block = collection.find_one(None, option).await?;

		let onchain_last_block = api.blocks().at_latest().await?;
		let block_number = onchain_last_block.number();

		let mut state = Self {
			current_block: start_block,
			latest_block: block_number,
			db,
			tasks: vec![],
			api,
			finalize_delay,
			rpc,
		};

		if let Some(b) = last_block {
			state.current_block = b.height;
		}

		Ok(state)
	}

	pub fn register(&mut self, task: Task) {
		self.tasks.push(Box::new(task));
	}
}

mod worker_utils {
	use super::*;

	async fn save_processed_status(db: Database, block: block::Block) -> Result<InsertOneResult> {
		let collection: Collection<block::Block> = db.collection(block::Block::name().as_str());
		Ok(collection.insert_one(block, None).await?)
	}

	async fn get_onchain_latest_block(api: OnlineClient<PolkadotConfig>) -> Result<block::Block> {
		let onchain_last_block = api.blocks().at_latest().await?;
		Ok(block::Block {
			height: onchain_last_block.number(),
			hash: (&onchain_last_block.hash()).to_string(),
		})
	}

	pub async fn start(root_state: WorkerState, status: &mut WorkerRunningStatus) -> Result<()> {
		let share_state = Arc::new(Mutex::new(root_state));
		status.enabled = true;
		if !status.running {
			// let t1 = tokio::spawn(run(state_root.clone()));
			let t2 = tokio::spawn(refetch_latest_block(share_state.clone(), &status, None));
			// let (r1, r2) = (t1.await?, t2.await?);
			t2.await?;
		}
		status.running = false;
		Ok(())
	}
	pub async fn stop(status: &mut WorkerRunningStatus) -> Result<()> {
		status.enabled = false;
		Ok(())
	}

	async fn refetch_latest_block(
		state: WorkerStateSync,
		status: &WorkerRunningStatus,
		delay_in_ms: Option<u64>,
	) -> Result<()> {
		println!("here");
		let mut state = state.lock().await;
		let delay_in_ms: u64 = delay_in_ms.unwrap_or(1000);
		println!("here1");

		while status.enabled {
			println!("here");

			let block = get_onchain_latest_block(state.api.clone()).await?;
			state.latest_block = block.height;
			println!("{}", block.height);

			// sleep(Duration::from_millis(delay_in_ms)).await;
		}
		Ok(())
	}

	async fn run(state_root: WorkerStateSync, status: &mut WorkerRunningStatus) -> Result<()> {
		println!("run here");
		let mut state = state_root.lock().await;
		let mut rs;
		while status.enabled {
			status.running = true;
			log::debug!("Begin process block {}", state.current_block);
			println!("Begin process block {}", state.current_block);

			rs = process_block(&mut state).await;
			if state.current_block < state.latest_block {
				match &rs {
					Err(err) => {
						log::warn!("{}", err);
					},
					Ok(block) => {
						log::debug!("Process block {} successfully {}", block.height, block.hash);
						state.current_block += 1;
					},
				}
			}
			tokio::time::sleep(Duration::from_millis(100)).await;
		}
		status.running = false;
		Ok(())
	}

	async fn process_block(state: &mut MutexGuard<'_, WorkerState>) -> Result<Block> {
		let block_number = state.current_block;
		let block_hash = state
			.api
			.rpc()
			.block_hash(Some(block_number.into()))
			.await?
			.expect(format!("Fail to get block hash of block {}", block_number).as_str());

		// Get events for the latest block:
		let events = state.api.events().at(block_hash).await?;
		for ev in events.iter() {
			let ev = ev?;
			log::info!("{}:{}", ev.pallet_name(), ev.variant_name());
		}

		Ok(Block {
			hash: block_hash.to_string(),
			height: block_number,
		})
	}
}

async fn on_new_seed(event: &EventDetails<PolkadotConfig>) {
	let new_seed = event.as_event::<gafi::game_randomness::events::NewSeed>();
	match new_seed {
		Ok(seed) =>
			if let Some(e) = seed {
				println!(" seed block: {:?}", e.block_number);
				println!(" seed seed: {:?}", e.seed);
			},
		Err(err) => {
			println!(" err: {:?}", err);
		},
	}
}

#[tokio::test]
async fn test() -> Result<()> {
	env_logger::init_from_env("info");
	println!("test");
	let db = shared::tests::utils::get_database().await;
	let mut worker_state = WorkerState::new(db, None, None, None).await?;

	let task = Task::new("NewSeed", "GameRandomness", move |ev| {
		Box::pin(on_new_seed(ev))
	});
	worker_state.register(task);
	let mut status = WorkerRunningStatus {
		enabled: Arc::new(true),
		running: Arc::new(false),
	};

	worker_utils::start(worker_state, &mut status).await?;

	Ok(())
}
