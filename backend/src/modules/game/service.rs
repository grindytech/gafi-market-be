use super::dto::{GameDTO, QueryFindGame};
use crate::common::{ErrorResponse, Page, QueryPage};
use actix_web::Result;
use futures_util::TryStreamExt;
use shared::{
	constant::EMPTY_STR,
	models::{self, game::Game},
	utils::{get_filter_option, get_total_page},
};
/* use futures::stream::StreamExt; */
use log::info;
use mongodb::{
	bson::{self, doc, Bson, Document},
	options, Collection, Cursor, Database,
};

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
	let query_find = doc! {
		"$or":
		[
			doc! {"game_id":params.query.game_id},
			doc! {"is_verified":params.query.is_verified},
			doc! {"owner":params.query.owner},
			doc! {"category":params.query.category},

		]
	};

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
