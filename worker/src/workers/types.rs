use futures::future::BoxFuture;
use mongodb::Database;
use shared::{types::Result, Block};
use subxt::{events::EventDetails, OnlineClient, PolkadotConfig};

/// - ev: a reference to an `OnchainEvent` which represents the event data.
/// - db: a reference to a `Database` which represents the database connection.
/// - api: a reference to an `RpcClient` which represents the remote procedure call client.
/// - block: a `Block` type object representing the current block.
/// - extrinsic_index: an optional `i32` representing the index of the extrinsic.
pub struct HandleParams<'a> {
	pub ev: &'a OnchainEvent,
	pub db: &'a Database,
	pub api: &'a RpcClient,
	pub block: Block,
	pub extrinsic_index: Option<i32>,
}
pub type OnchainEvent = EventDetails<PolkadotConfig>;
pub type RpcClient = OnlineClient<PolkadotConfig>;

/// - key: A string representing the event name to handle. The format of the key is PalletName:EventName. 
/// - runner: A function that will be executed when the event specified by the key occurs.
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