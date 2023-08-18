use crate::gafi;

use super::*;
use async_trait::async_trait;
use subxt::{events::EventDetails, OnlineClient, PolkadotConfig};

pub struct Worker {
	tasks: Vec<Task>,
}

impl Worker {
	async fn init(&mut self) {}

	fn register(&mut self, task: Task) {
		self.tasks.push(task);
	}

	async fn run(&self) -> Result<(), subxt::Error> {
		// Create a client to use:
		let api =
			OnlineClient::<PolkadotConfig>::from_url("wss://rpc-testnet.gafi.network:443").await?;

		// Get events for the latest block:
		let events = api.events().at_latest().await?;
		for event in events.iter() {
			let event = event?;
			for task in &self.tasks {
				task.clone().run(&event);
			}
		}
		Ok(())
	}
}

async fn on_new_seed(event: EventDetails<PolkadotConfig>) {
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
async fn test() {
	let mut worker = Worker { tasks: vec![] };
	let mut task = Task {
		event_name: "NewSeed".to_string(),
		pallet_name: "GameRandomness".to_string(),
		runner: None,
	};
	task.set_runner(on_new_seed);
	worker.register(task);
}
