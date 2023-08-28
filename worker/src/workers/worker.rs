use super::*;
use mongodb::{bson::doc, options::FindOneOptions, results::InsertOneResult, Collection, Database};
use shared::{block, types::Result, BaseDocument, Block};
use subxt::{events::Phase, OnlineClient, PolkadotConfig};
use tokio::time::Duration;

fn get_db_track_name(name: &str) -> String {
	format!("{}_{}", name, block::Block::name())
}

pub struct WorkerState {
	tasks: Vec<Task>,
	current_block: u32,
	latest_block: u32,
	db: Database,
	api: RpcClient,
	rpc: String,
	enabled: bool,
	running: bool,
	max_batch: u32,
}

impl WorkerState {
	pub async fn new(
		db: Database,
		start_block: u32,
		rpc: String,
		max_batch: Option<u32>,
		name: &str,
	) -> Result<Self> {
		let max_batch = max_batch.unwrap_or(1000);
		let api = OnlineClient::<PolkadotConfig>::from_url(&rpc).await?;

		let sort = doc! {"height": -1};
		let option = FindOneOptions::builder().sort(sort).build();
		let collection: Collection<block::Block> = db.collection(get_db_track_name(name).as_str());
		let last_block = collection.find_one(None, option).await?;
		log::info!("db block {:?}", last_block);
		let onchain_last_block = api.blocks().at_latest().await?;
		let block_number = onchain_last_block.number();

		let mut state = Self {
			current_block: start_block,
			latest_block: block_number,
			db,
			tasks: vec![],
			api,
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

	pub fn add_tasks(&mut self, tasks: &mut Vec<Task>) {
		self.tasks.append(tasks);
	}
}

pub struct Worker {
	state: WorkerState,
	pub name: String,
}
impl Worker {
	pub async fn new(
		name: String,
		db: Database,
		start_block: u32,
		rpc: String,
		max_batch: Option<u32>,
	) -> Result<Self> {
		let state = WorkerState::new(db, start_block, rpc, max_batch, &name).await?;
		Ok(Self { name, state })
	}

	pub fn add_tasks(&mut self, tasks: &mut Vec<Task>) {
		self.state.add_tasks(tasks);
	}

	async fn save_processed_status(
		db: &Database,
		block: block::Block,
		name: String,
	) -> Result<InsertOneResult> {
		let collection: Collection<block::Block> = db.collection(&get_db_track_name(&name));
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
		tasks: &Vec<Task>,
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

			let extrinsic_index: Option<i32>;
			match ev.phase() {
				Phase::ApplyExtrinsic(i) => extrinsic_index = Some(i as i32),
				Phase::Finalization => extrinsic_index = Some(-1),
				Phase::Initialization => extrinsic_index = Some(-2),
			}
			// log::debug!("phase {:?}", ev.phase());
			// if let Ok(ev) = ev.as_root_event::<gafi::Event>() {
			// 	log::debug!("{ev:?}");
			// } else {
			// 	log::warn!("<Cannot decode event>");
			// }
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
						extrinsic_index,
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

		let end_block = if (i64::from(state.latest_block) - i64::from(state.current_block)) >
			i64::from(state.max_batch)
		{
			state.current_block + state.max_batch
		} else {
			state.latest_block
		};

		for block_number in state.current_block..end_block + 1 {
			log::info!(
				"[{}] Begin process block {}",
				self.name,
				state.current_block
			);
			let block =
				Self::process_block(&state.api, &state.db, &state.tasks, block_number).await?;
			log::info!(
				"[{}] Process block {} successfully {}",
				self.name,
				block.height,
				block.hash
			);
			Self::save_processed_status(&state.db, block.clone(), self.name.clone()).await?;
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
					let new_client =
						OnlineClient::<PolkadotConfig>::from_url(&self.state.rpc).await;
					match new_client {
						Ok(client) => {
							self.state.api = client;
						},
						Err(err) => {
							log::error!("[{}] Err: {:?}", self.name, err);
						},
					}
					log::error!("[{}] Err: {:?}", self.name, err);
				},
			}
			if delay_loop > 0 {
				tokio::time::sleep(Duration::from_millis(delay_loop)).await;
			}
		}
		Ok(())
	}
}
