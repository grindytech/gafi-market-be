pub use shared::types::Result;
use shared::constant::{EVENT_COLLECTION_ADDED, EVENT_COLLECTION_REMOVED, EVENT_GAME_CREATED};

use crate::{
	gafi::{self},
	services::game_service,
	workers::{HandleParams, EventHandle},
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
	]
}
