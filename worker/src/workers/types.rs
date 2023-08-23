use std::pin::Pin;

use futures::{future::BoxFuture, Future};
use mongodb::Database;
use shared::{types::Result, Block};
use subxt::{events::EventDetails, OnlineClient, PolkadotConfig};

pub struct HandleParams<'a> {
	pub ev: &'a OnchainEvent,
	pub db: &'a Database,
	pub api: &'a RpcClient,
	pub block: Block,
	// tx
}
pub type OnchainEvent = EventDetails<PolkadotConfig>;
pub type RpcClient = OnlineClient<PolkadotConfig>;
pub struct Task {
	pub key: String,
	pub runner: Box<dyn Fn(HandleParams) -> BoxFuture<Result<()>> + Send + Sync>,
}
impl Task {
	pub fn new<Func>(key: &str, func: Func) -> Self
	where
		Func: Fn(HandleParams) -> BoxFuture<Result<()>> + Send + Sync + 'static,
	{
		Self {
			key: key.to_string(),
			runner: Box::new(func),
		}
	}
	pub async fn run(&self, params: HandleParams<'_>) -> Result<()> {
		(self.runner)(params).await
	}
}

// #[async_trait]
// pub trait IWorker {
// 	async fn init(&mut self);
// 	fn register(&mut self, task: Task);
// 	async fn run(&self);
// }
