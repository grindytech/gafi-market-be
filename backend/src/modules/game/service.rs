use super::dto::GameDTO;
use crate::common::ErrorResponse;
use actix_web::Result;
use futures_util::TryStreamExt;
use shared::{models::{self, game::Game}, BaseDocument};
/* use futures::stream::StreamExt; */
use log::info;
use mongodb::{
	bson::{self, doc, Bson, Document},
	options, Collection, Cursor, Database,
};
pub async fn get_game_by_id(
	game_id: &String,
	db: Database,
) -> Result<Option<Game>, mongodb::error::Error> {
	let col: Collection<Game> = db.collection(models::Game::name().as_str());
	let filter = doc! {"game_id":game_id};
	col.find_one(filter, None).await
}

pub async fn find_games_account(
	address: &String,
	db: Database,
) -> Result<Option<Vec<GameDTO>>, mongodb::error::Error> {
	let filter = doc! {"owner":address};
	let col: Collection<Game> = db.collection(models::Game::name().as_str());
	/* let option = options::FindOptions::default(); */
	let mut cursor = col.find(filter, None).await?;

	let mut list_games: Vec<GameDTO> = Vec::new();

	while let Some(game) = cursor.try_next().await? {
		list_games.push(game.into())
	}

	Ok(Some(list_games))
}
