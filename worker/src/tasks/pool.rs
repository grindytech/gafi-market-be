use crate::{
	gafi::{
		self,
		runtime_types::{gafi_support::game::types::PoolType, pallet_game::types::PoolDetails},
	},
	workers::{HandleParams, Task},
};
use mongodb::{
	bson::{doc, DateTime, Document},
	options::UpdateOptions,
};
use shared::{types::Result, BaseDocument, Pool};

async fn on_pool_created(params: HandleParams<'_>) -> Result<()> {
	let event_parse = params.ev.as_event::<gafi::game::events::MiningPoolCreated>()?;
	if let Some(ev) = event_parse {
		let pool_type = match ev.pool_type {
			PoolType::Dynamic => "Dynamic",
			PoolType::Stable => "Stable",
		};

		let pool_db = params.db.collection::<Pool>(Pool::name().as_str());
		let option = UpdateOptions::builder().upsert(true).build();
		let query = doc! {"pool_id": ev.pool.to_string()};

		let pool_storage_address = gafi::storage().game().pool_of(ev.pool);
		let pool_detail = params
			.api
			.storage()
			.at_latest()
			.await?
			.fetch(&pool_storage_address)
			.await?
			.expect("Cannot get pool detail info");

		let loot_table = ev
			.table
			.iter()
			.map(|item| {
				let nft_loot = match &item.maybe_nft {
					Some(nft) => Some(doc! {
						"collection": nft.collection.to_string(),
						"item": nft.item.to_string(),
					}),
					None => None,
				};
				doc! {
					"nft": nft_loot,
					"weight": item.weight,
				}
			})
			.collect::<Vec<Document>>();

		let mint_type = match pool_detail.mint_settings.mint_type {
			gafi::runtime_types::gafi_support::game::types::MintType::Public => "Public",
			gafi::runtime_types::gafi_support::game::types::MintType::HolderOf(_) => "HolderOf",
		};
		let upsert = doc! {"$set": {
			"pool_id": ev.pool.to_string(),
			"type_pool": pool_type,
			"loot_table": loot_table.clone(),
		  "owner": hex::encode(ev.who.0),
			"admin": hex::encode(pool_detail.admin.0),
			"mint_type":	mint_type,
			"minting_fee": pool_detail.mint_settings.price.to_string(),
			"begin_at": pool_detail.mint_settings.start_block,
			"end_at":  pool_detail.mint_settings.end_block,
			"owner_deposit": pool_detail.owner_deposit.to_string(),
			"updated_at": DateTime::now(),
			"create_at": DateTime::now(),
		}};

		pool_db.update_one(query, upsert, option).await?;
		log::info!(
			"MiningPoolCreated created {} {}, who: {}, loot_table: {:?}",
			ev.pool.to_string(),
			pool_type,
			hex::encode(ev.who.0),
			loot_table,
		);
	}
	Ok(())
}

pub fn tasks() -> Vec<Task> {
	vec![Task::new("Game:MiningPoolCreated", move |params| {
		Box::pin(on_pool_created(params))
	})]
}
