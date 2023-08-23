use crate::gafi;

use super::*;
use mongodb::{bson::doc, options::FindOneOptions, results::InsertOneResult, Collection, Database};
use shared::{block, types::Result, BaseDocument, Block};
use subxt::{OnlineClient, PolkadotConfig};
use tokio::time::Duration;

pub struct WorkerState {
	tasks: Vec<Box<Task>>,
	current_block: u32,
	latest_block: u32,
	db: Database,
	api: RpcClient,
	finalize_delay: u32,
	rpc: String,
	enabled: bool,
	running: bool,
	max_batch: u32,
}

impl WorkerState {
	pub async fn new(
		db: Database,
		finalize_delay: Option<u32>,
		start_block: Option<u32>,
		rpc: Option<String>,
		max_batch: Option<u32>,
	) -> Result<Self> {
		let max_batch = max_batch.unwrap_or(1000);
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
			enabled: false,
			running: false,
			max_batch,
		};

		if let Some(b) = last_block {
			state.current_block = b.height + 1;
		}

		Ok(state)
	}

	pub fn register(&mut self, task: Task) {
		self.tasks.push(Box::new(task));
	}
}

pub struct Worker {
	state: WorkerState,
}
impl Worker {
	pub async fn new(
		db: Database,
		finalize_delay: Option<u32>,
		start_block: Option<u32>,
		rpc: Option<String>,
		max_batch: Option<u32>,
	) -> Result<Self> {
		let state = WorkerState::new(db, finalize_delay, start_block, rpc, max_batch).await?;
		Ok(Self { state })
	}

	pub fn register(&mut self, task: Task) {
		self.state.register(task)
	}

	async fn save_processed_status(db: &Database, block: block::Block) -> Result<InsertOneResult> {
		let collection: Collection<block::Block> = db.collection(block::Block::name().as_str());
		Ok(collection.insert_one(block, None).await?)
	}

	async fn get_onchain_latest_block(api: &OnlineClient<PolkadotConfig>) -> Result<block::Block> {
		let onchain_last_block = api.blocks().at_latest().await?;
		Ok(block::Block {
			height: onchain_last_block.number(),
			hash: hex::encode(onchain_last_block.hash().0),
		})
	}

	async fn process_block(
		api: &RpcClient,
		db: &Database,
		tasks: &Vec<Box<Task>>,
		block_number: u32,
	) -> Result<Block> {
		let block_hash = api
			.rpc()
			.block_hash(Some(block_number.into()))
			.await?
			.expect(format!("Fail to get block hash of block {}", block_number).as_str());
		let block_hash_str = hex::encode(block_hash.0.to_vec());

		let events = api.events().at(block_hash).await?;
		for ev in events.iter() {
			let ev = ev?;
			if let Ok(ev) = ev.as_root_event::<gafi::Event>() {
				log::debug!("{ev:?}");
			} else {
				log::warn!("<Cannot decode event>");
			}
			for task in tasks {
				if task.key == format!("{}:{}", ev.pallet_name(), ev.variant_name()) {
					task.run(HandleParams {
						ev: &ev,
						db,
						api,
						block: Block {
							height: block_number,
							hash: block_hash_str.clone(),
						},
					})
					.await?; //TODO process in multi threads
				}
			}
		}

		Ok(Block {
			hash: block_hash_str,
			height: block_number,
		})
	}

	pub async fn stop(&mut self) -> Result<()> {
		let state = &mut self.state;
		state.enabled = false;
		Ok(())
	}
	/// return false if worker disabled
	async fn run(&mut self) -> Result<bool> {
		let mut state = &mut self.state;
		state.running = true;

		let end_block = if (state.latest_block - state.current_block) > state.max_batch {
			state.current_block + state.max_batch
		} else {
			state.latest_block
		};

		for block_number in state.current_block..end_block {
			log::info!("Begin process block {}", state.current_block);
			let block =
				Self::process_block(&state.api, &state.db, &state.tasks, block_number).await?;
			log::info!("Process block {} successfully {}", block.height, block.hash);
			Self::save_processed_status(&state.db, block.clone()).await?;
			state.current_block += 1;

			if !state.enabled {
				break
			}
		}

		let latest_block = Self::get_onchain_latest_block(&state.api).await?;
		state.latest_block = latest_block.height;
		state.running = false;
		Ok(state.enabled.clone())
	}

	pub async fn start(&mut self, delay_loop: u64) -> Result<()> {
		let state = &mut self.state;
		if state.enabled {
			return Ok(())
		}
		state.enabled = true;
		loop {
			let rs = self.run().await;
			match rs {
				Ok(enabled) =>
					if enabled == false {
						break
					},
				Err(err) => {
					log::error!("Err: {}", err);
				},
			}
			if delay_loop > 0 {
				tokio::time::sleep(Duration::from_millis(delay_loop)).await;
			}
		}
		Ok(())
	}
}

async fn on_new_seed(params: HandleParams<'_>) -> Result<()> {
	let new_seed = params.ev.as_event::<gafi::game_randomness::events::NewSeed>()?;
	if let Some(e) = new_seed {
		log::info!(" seed block: {:?}", e.block_number);
		log::info!(" seed seed: {:?}", e.seed);
	};
	Ok(())
}

#[tokio::test]
async fn test() -> Result<()> {
	env_logger::init_from_env("info");
	let db = shared::tests::utils::get_database().await;
	let mut worker = Worker::new(db, None, Some(127200), None, None).await?;

	let task = Task::new("GameRandomness:NewSeed", move |params| {
		Box::pin(on_new_seed(params))
	});
	worker.register(task);

	let _ = worker.start(1000).await;

	Ok(())
}