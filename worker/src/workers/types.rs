use std::pin::Pin;

use async_trait::async_trait;
use futures::{future::BoxFuture, Future};
use subxt::{events::EventDetails, PolkadotConfig};

pub struct Task {
	pub event_name: String,
	pub pallet_name: String,
	// pub process: Box<dyn Fn(EventDetails<PolkadotConfig>) -> BoxFuture<'static, ()>>,
	pub runner: Option<
		Box<
			dyn FnOnce(&EventDetails<PolkadotConfig>) -> Pin<Box<dyn Future<Output = ()> + Send>>
				+ Send,
		>,
	>,
}
impl Task {
	pub async fn run(self, ev: &EventDetails<PolkadotConfig>) {
		(self.runner.unwrap())(ev).await;
	}

	pub fn set_runner<Func, Fut>(&mut self, func: Func)
	where
		Func: Send + 'static + FnOnce(&EventDetails<PolkadotConfig>) -> Fut,
		Fut: Send + 'static + Future<Output = ()>,
	{
		self.runner = Some(Box::new(move |ev| Box::pin(func(ev))));
	}
}

// #[async_trait]
// pub trait IWorker {
// 	async fn init(&mut self);
// 	fn register(&mut self, task: Task);
// 	async fn run(&self);
// }
