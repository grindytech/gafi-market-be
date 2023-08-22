use mongodb::Database;
pub use shared::types::Result;
use shared::{BaseDocument, NFT};

use crate::{
	gafi,
	workers::{HandleParams, OnchainEvent, RpcClient, Task},
};

pub async fn on_mint_nft(params: HandleParams<'_>) -> Result<()> {
	let nft_db = params.db.collection::<NFT>(NFT::name().as_str());
	// let nfts_insert = vec![];
	let event_parse = params.ev.as_event::<gafi::game::events::Minted>()?;
	if let Some(nft) = event_parse {
		// for item in nft.nfts {
		// 	nfts_insert.push(NFT{
		// 		// collection_id: nft.pool
		// 	});
		// }
		// nft.nfts
		// nft.owner.to_string();
	};
	Ok(())
}
pub fn on_mint_nft_task() -> Task {
	let task = Task::new("Nfts:Minted", move |params| Box::pin(on_mint_nft(params)));
	task
}
