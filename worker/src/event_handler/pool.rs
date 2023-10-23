use crate::{
	gafi::{self, runtime_types::gafi_support::game::types::PoolType},
	services::{self, pool_service},
	workers::{EventHandle, HandleParams},
};
use mongodb::bson::DateTime;
use shared::{
	constant::{EVENT_MINING_POOL_CREATED, EVENT_POOL_METADATA_CLEARED, EVENT_POOL_METADATA_SET},
	types::Result,
	LootTable, LootTableNft, Pool,
};

async fn on_pool_created(params: HandleParams<'_>) -> Result<()> {
	let event_parse = params.ev.as_event::<gafi::game::events::MiningPoolCreated>()?;
	if let Some(ev) = event_parse {
		let pool_type = match ev.pool_type {
			PoolType::Dynamic => "Dynamic",
			PoolType::Stable => "Stable",
		};

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
					Some(nft) => Some(LootTableNft {
						collection: nft.collection.to_string(),
						item: nft.item.to_string(),
					}),
					None => None,
				};
				LootTable {
					nft: nft_loot,
					weight: item.weight,
				}
			})
			.collect::<Vec<LootTable>>();

		let mint_type = match pool_detail.mint_settings.mint_type {
			gafi::runtime_types::gafi_support::game::types::MintType::Public => "Public",
			gafi::runtime_types::gafi_support::game::types::MintType::HolderOf(_) => "HolderOf",
		};
		let config = shared::config::Config::init();
		let mining_fee = shared::utils::string_decimal_to_number(
			&pool_detail.mint_settings.price.to_string(),
			config.chain_decimal as i32,
		);
		let pool = Pool {
			admin: hex::encode(pool_detail.admin.0),
			start_block: pool_detail.mint_settings.start_block.unwrap_or(0).into(),
			created_at: DateTime::now().timestamp_millis(),
			end_block: pool_detail.mint_settings.end_block.unwrap_or(0).into(),
			id: None,
			title: None,
			description: None,
			loot_table: loot_table.clone(),
			mint_type: mint_type.to_string(),
			minting_fee: mining_fee.parse()?,
			owner: hex::encode(ev.who.0),
			owner_deposit: pool_detail.owner_deposit.to_string(),
			pool_id: ev.pool.to_string(),
			type_pool: pool_type.to_string(),
			updated_at: DateTime::now().timestamp_millis(),
		};

		pool_service::upsert_pool(pool, params.db).await?;
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
async fn on_pool_metadata_set(params: HandleParams<'_>) -> Result<()> {
	let event_parse = params.ev.as_event::<gafi::game::events::PoolSetMetadata>()?;

	if let Some(ev) = event_parse {
		let data = String::from_utf8(ev.data.0).ok();
		if let Some(metadata) = data {
			services::pool_service::update_metadata(metadata, ev.pool, params.db).await?;
		};
	};
	Ok(())
}
async fn on_pool_metadata_cleared(params: HandleParams<'_>) -> Result<()> {
	let event_parse = params.ev.as_event::<gafi::game::events::PoolSetMetadataCleared>()?;
	if let Some(ev) = event_parse {
		services::pool_service::clear_metadata(ev.pool, params.db).await?;
	};
	Ok(())
}
pub fn tasks() -> Vec<EventHandle> {
	vec![
		EventHandle::new(EVENT_MINING_POOL_CREATED, move |params| {
			Box::pin(on_pool_created(params))
		}),
		EventHandle::new(EVENT_POOL_METADATA_SET, move |params| {
			Box::pin(on_pool_metadata_set(params))
		}),
		EventHandle::new(EVENT_POOL_METADATA_CLEARED, move |params| {
			Box::pin(on_pool_metadata_cleared(params))
		}),
	]
}
