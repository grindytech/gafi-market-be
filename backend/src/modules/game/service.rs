use super::dto::{GameDTO, QueryFindGame};
use crate::common::{
	utils::{get_filter_option, get_total_page},
	Page, QueryPage,
};
use actix_web::Result;
use futures_util::TryStreamExt;

use shared::{
	constant::EMPTY_STR,
	models::{self, game::Game},
	BaseDocument,
};

use mongodb::{bson::doc, Collection, Database};

use crate::common::DBQuery;

// Find Game Detail By Game ID
pub async fn find_game_by_id(
	game_id: &String,
	db: Database,
) -> Result<Option<GameDTO>, mongodb::error::Error> {
	let col: Collection<Game> = db.collection(models::game::Game::name().as_str());
	let filter = doc! {"game_id":game_id};

	if let Ok(Some(game_detail)) = col.find_one(filter, None).await {
		Ok(Some(game_detail.into()))
	} else {
		Ok(None)
	}
}

//Find List Game By Address Account
pub async fn find_games_by_query(
	params: QueryPage<QueryFindGame>,
	db: Database,
) -> Result<Option<Page<GameDTO>>, mongodb::error::Error> {
	let col: Collection<Game> = db.collection(models::game::Game::name().as_str());

	let query_find = params.query.to_doc();
	let filter_option = get_filter_option(params.order_by, params.desc).await;

	let mut cursor = col.find(query_find, filter_option).await?;
	let mut list_games: Vec<GameDTO> = Vec::new();
	while let Some(game) = cursor.try_next().await? {
		list_games.push(game.into())
	}

	let total = get_total_page(list_games.len(), params.size).await;
	Ok(Some(Page::<GameDTO> {
		data: list_games,
		message: EMPTY_STR.to_string(),
		page: params.page,
		size: params.size,
		total,
	}))
}
