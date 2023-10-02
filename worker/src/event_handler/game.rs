use shared::constant::{
	EVENT_COLLECTION_ADDED, EVENT_COLLECTION_REMOVED, EVENT_GAME_CREATED,
	EVENT_GAME_METADATA_CLEARED, EVENT_GAME_SET_METADATA,
};
pub use shared::types::Result;

use crate::{
	gafi::{self},
	services::{self, game_service},
	workers::{EventHandle, HandleParams},
};

async fn on_collection_added(params: HandleParams<'_>) -> Result<()> {
	let event_parse = params.ev.as_event::<gafi::game::events::CollectionAdded>()?;
	if let Some(ev) = event_parse {
		game_service::add_collection(&ev.game.to_string(), &ev.collection.to_string(), params.db)
			.await?;
	}
	Ok(())
}

async fn on_collection_removed(params: HandleParams<'_>) -> Result<()> {
	let event_parse = params.ev.as_event::<gafi::game::events::CollectionRemoved>()?;
	if let Some(ev) = event_parse {
		game_service::remove_collection(
			&ev.game.to_string(),
			&ev.collection.to_string(),
			params.db,
		)
		.await?;
	}
	Ok(())
}

async fn on_game_created(params: HandleParams<'_>) -> Result<()> {
	let event_parse = params.ev.as_event::<gafi::game::events::GameCreated>()?;
	if let Some(game) = event_parse {
		game_service::upsert_game_without_metadata(
			&game.game.to_string(),
			&hex::encode(game.who.0),
			params.db,
		)
		.await?;
	}
	Ok(())
}
async fn on_game_set_metadata(params: HandleParams<'_>) -> Result<()> {
	let event_parse = params.ev.as_event::<gafi::game::events::GameSetMetadata>()?;
	if let Some(ev) = event_parse {
		let data = String::from_utf8(ev.data.0).ok();
		if let Some(metadata) = data {
			services::game_service::update_metadata(metadata, ev.game, params.db).await?;
		}
	};
	Ok(())
}
async fn on_game_metadata_cleared(params: HandleParams<'_>) -> Result<()> {
	let event_parse = params.ev.as_event::<gafi::game::events::GameMetadataCleared>()?;
	if let Some(ev) = event_parse {
		services::game_service::clear_metadata(&ev.game.to_string(), params.db).await?;
	};
	Ok(())
}
pub fn tasks() -> Vec<EventHandle> {
	vec![
		EventHandle::new(EVENT_GAME_CREATED, move |params| {
			Box::pin(on_game_created(params))
		}),
		EventHandle::new(EVENT_COLLECTION_ADDED, move |params| {
			Box::pin(on_collection_added(params))
		}),
		EventHandle::new(EVENT_COLLECTION_REMOVED, move |params| {
			Box::pin(on_collection_removed(params))
		}),
		EventHandle::new(EVENT_GAME_SET_METADATA, move |params| {
			Box::pin(on_game_set_metadata(params))
		}),
		EventHandle::new(EVENT_GAME_METADATA_CLEARED, move |params| {
			Box::pin(on_game_metadata_cleared(params))
		}),
	]
}
