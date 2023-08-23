use mongodb::{
	bson::{doc, DateTime},
	options::UpdateOptions,
};
pub use shared::types::Result;
use shared::{BaseDocument, Game};

use crate::{
	gafi::game::events::GameCreated,
	workers::{HandleParams, Task},
};

async fn on_game_created(params: HandleParams<'_>) -> Result<()> {
	let event_parse = params.ev.as_event::<GameCreated>()?;
	if let Some(game) = event_parse {
		let game_db: mongodb::Collection<Game> =
			params.db.collection::<Game>(Game::name().as_str());

		let option = UpdateOptions::builder().upsert(true).build();
		let query = doc! {"game_id": game.game.to_string()};
		let new_game = doc! { "$set": {
		"game_id": game.game.to_string(),
		"owner": hex::encode(game.who.0),
		"created_at": DateTime::now(),
		"updated_at": DateTime::now()
		  }
		  };
		game_db.update_one(query, new_game, Some(option)).await?;
		log::info!("Game created {} {}", game.game, game.who);
	}
	Ok(())
}
pub fn tasks() -> Vec<Task> {
	vec![Task::new("Game:GameCreated", move |params| {
		Box::pin(on_game_created(params))
	})]
}
