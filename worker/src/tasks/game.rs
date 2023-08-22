use mongodb::Database;
pub use shared::types::Result;
use shared::{BaseDocument, Game};

use crate::{
	gafi::{self, game::events::GameCreated},
	workers::{HandleParams, OnchainEvent, RpcClient, Task},
};

pub async fn on_game_created(params: HandleParams<'_>) -> Result<()> {
	let game_db: mongodb::Collection<Game> = params.db.collection::<Game>(Game::name().as_str());
	let event_parse = params.ev.as_event::<GameCreated>()?;
	if let Some(game) = event_parse {
    // let g = Game{
    //   game_id: game.game.to_string(),
    //   owner: hex::encode(game.who.0),
    //   banner_url:None,
    //   category: None,
    //   create_at:
    // }
    // game_db.insert_one(doc, options)
		print!("game {} {}", game.game, game.who);
	}
	Ok(())
}
pub fn on_game_created_task() -> Task {
	let task = Task::new("Game:GameCreated", move |params| {
		Box::pin(on_game_created(params))
	});
	task
}
