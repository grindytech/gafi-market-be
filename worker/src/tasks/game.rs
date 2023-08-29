use mongodb::{
	bson::{doc, DateTime, Document},
	options::UpdateOptions,
};
pub use shared::types::Result;
use shared::{
	constant::{EVENT_COLLECTION_ADDED, EVENT_COLLECTION_REMOVED, EVENT_GAME_CREATED},
	BaseDocument, Game, NFTCollection,
};

use crate::{
	gafi::{self},
	workers::{HandleParams, Task},
};

async fn on_collection_added(params: HandleParams<'_>) -> Result<()> {
	let event_parse = params.ev.as_event::<gafi::game::events::CollectionAdded>()?;
	if let Some(ev) = event_parse {
		let game_db = params.db.collection::<Game>(Game::name().as_str());
		let collection_db = params.db.collection::<NFTCollection>(NFTCollection::name().as_str());
		let game = game_db
			.find_one(
				doc! {
					"game_id": ev.game.to_string(),
				},
				None,
			)
			.await?
			.unwrap();
		let collection = collection_db
			.find_one(
				doc! {
					"collection_id": ev.collection.to_string(),
				},
				None,
			)
			.await?
			.unwrap();
		let mut collections: Vec<String> = match game.collections {
			Some(c) => c,
			None => vec![],
		};
		let mut games: Vec<String> = match collection.games {
			Some(g) => g,
			None => vec![],
		};
		collections.push(ev.collection.to_string());
		games.push(ev.game.to_string());
		game_db
			.update_one(
				doc! {
					"game_id": ev.game.to_string(),
				},
				doc! {
					"collections": collections,
				},
				None,
			)
			.await?;
		collection_db
			.update_one(
				doc! {
					"collection_id": ev.collection.to_string(),
				},
				doc! {
					"games": games,
				},
				None,
			)
			.await?;
	}
	Ok(())
}

async fn on_collection_removed(params: HandleParams<'_>) -> Result<()> {
	let event_parse = params.ev.as_event::<gafi::game::events::CollectionRemoved>()?;
	if let Some(ev) = event_parse {
		let game_db = params.db.collection::<Game>(Game::name().as_str());
		let collection_db = params.db.collection::<NFTCollection>(NFTCollection::name().as_str());
		let game = game_db
			.find_one(
				doc! {
					"game_id": ev.game.to_string(),
				},
				None,
			)
			.await?
			.unwrap();
		let collection = collection_db
			.find_one(
				doc! {
					"collection_id": ev.collection.to_string(),
				},
				None,
			)
			.await?
			.unwrap();
		let mut collections: Vec<String> = match game.collections {
			Some(c) => c.into_iter().filter(|c| *c != ev.collection.to_string()).collect(),
			None => vec![],
		};
		let mut games: Vec<String> = match collection.games {
			Some(g) => g.into_iter().filter(|g| *g != ev.game.to_string()).collect(),
			None => vec![],
		};
		collections.push(ev.collection.to_string());
		games.push(ev.game.to_string());
		game_db
			.update_one(
				doc! {
					"game_id": ev.game.to_string(),
				},
				doc! {
					"collections": collections,
				},
				None,
			)
			.await?;
		collection_db
			.update_one(
				doc! {
					"collection_id": ev.collection.to_string(),
				},
				doc! {
					"games": games,
				},
				None,
			)
			.await?;
	}
	Ok(())
}

async fn on_game_created(params: HandleParams<'_>) -> Result<()> {
	let event_parse = params.ev.as_event::<gafi::game::events::GameCreated>()?;
	if let Some(game) = event_parse {
		let game_db: mongodb::Collection<Game> =
			params.db.collection::<Game>(Game::name().as_str());

		let option = UpdateOptions::builder().upsert(true).build();
		let query = doc! {"game_id": game.game.to_string()};
		let game_doc: Document = Game {
			banner_url: None,
			category: None,
			collections: None,
			created_at: Some(DateTime::now()),
			description: None,
			game_id: game.game.to_string(),
			id: None,
			is_verified: None,
			logo_url: None,
			name: None,
			owner: hex::encode(game.who.0),
			slug: None,
			social: None,
			updated_at: None,
		}
		.into();
		let new_game = doc! {
			"$set": game_doc,
		};
		game_db.update_one(query, new_game, Some(option)).await?;
		log::info!("Game created {} {}", game.game, game.who);
	}
	Ok(())
}
pub fn tasks() -> Vec<Task> {
	vec![
		Task::new(EVENT_GAME_CREATED, move |params| {
			Box::pin(on_game_created(params))
		}),
		Task::new(EVENT_COLLECTION_ADDED, move |params| {
			Box::pin(on_collection_added(params))
		}),
		Task::new(EVENT_COLLECTION_REMOVED, move |params| {
			Box::pin(on_collection_removed(params))
		}),
	]
}
