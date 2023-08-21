use std::pin::Pin;

use async_trait::async_trait;
use futures::{future::BoxFuture, Future};
use subxt::{events::EventDetails, PolkadotConfig};

pub struct Task {
	pub event_name: String,
	pub pallet_name: String,
	pub runner: Box<dyn Fn(&EventDetails<PolkadotConfig>) -> BoxFuture<()> + Send + Sync>,
}
impl Task {
	pub fn new<Func>(event_name: &str, pallet_name: &str, func: Func) -> Self
	where
		Func: Fn(&EventDetails<PolkadotConfig>) -> BoxFuture<()> + Send + Sync + 'static,
	{
		Self {
			event_name: event_name.to_string(),
			pallet_name: pallet_name.to_string(),
			runner: Box::new(func),
		}
	}
	pub async fn run(&self, ev: &EventDetails<PolkadotConfig>) {
		(self.runner)(ev).await;
	}
}

// #[async_trait]
// pub trait IWorker {
// 	async fn init(&mut self);
// 	fn register(&mut self, task: Task);
// 	async fn run(&self);
// }
