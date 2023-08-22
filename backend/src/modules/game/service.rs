use std::collections::HashMap;

use super::dto::{GameDTO, QueryFindGame};
use crate::common::{
	utils::{create_or_query, get_filter_option, get_total_page},
	ErrorResponse, Page, QueryPage,
};
use actix_web::Result;
use futures_util::TryStreamExt;
use serde_json::Value;
use shared::{
	constant::EMPTY_STR,
	models::{self, game::Game},
};
/* use futures::stream::StreamExt; */
use log::info;
use mongodb::{
	bson::{self, doc, Bson, Document},
	options, Collection, Cursor, Database,
};
//------------

// Find Game Detail By Game ID
pub async fn find_game_by_id(
	game_id: &String,
	db: Database,
) -> Result<Option<GameDTO>, mongodb::error::Error> {
	let col: Collection<Game> = db.collection(models::game::NAME);
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
	let col: Collection<Game> = db.collection(models::game::NAME);

	let mut criteria: HashMap<String, Value> = HashMap::new();
	criteria.insert("game_id".to_string(), Value::String(params.query.game_id));
	criteria.insert(
		"is_verified".to_string(),
		Value::Bool(params.query.is_verified),
	);
	criteria.insert("owner".to_string(), Value::String(params.query.owner));
	criteria.insert("category".to_string(), Value::String(params.query.category));

	let query_find = create_or_query(criteria).await;

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
