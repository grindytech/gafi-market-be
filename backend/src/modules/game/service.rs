use super::dto::GameDTO;
use crate::common::{GamePage, Page, QueryGame};
use futures_util::TryStreamExt;

use shared::{
	constant::EMPTY_STR,
	models::{self, game::Game},
	BaseDocument,
};

use mongodb::{bson::doc, Collection, Database};

use crate::common::DBQuery;

//Find List Game By Address Account
pub async fn find_games_by_query(
	params: QueryGame,
	db: Database,
) -> shared::Result<Option<GamePage>> {
	let col: Collection<Game> = db.collection(models::game::Game::name().as_str());

	let query_find = params.query.to_doc();

	let filter_match = doc! {
		"$match":query_find,
	};
	let paging = doc! {
	  "$facet":{
			"paginatedResults": [ { "$skip": params.skip() }, { "$limit": params.size() } ],
		  "totalCount": [ { "$count": "count" } ],
		},
	};
	let sort = doc! {
		"$sort":params.sort()
	};
	let mut cursor = col.aggregate(vec![filter_match, sort, paging], None).await?;
	let mut list_games: Vec<GameDTO> = Vec::new();
	let document = cursor.try_next().await?.ok_or("cursor try_next failed")?;
	let paginated_result = document.get_array("paginatedResults")?;

	paginated_result.into_iter().for_each(|rs| {
		let game_str = serde_json::to_string(&rs).expect("Failed Parse game to String");
		let game: Game = serde_json::from_str(&game_str).expect("Failed to Parse to NFT game");
		list_games.push(game.into());
	});
	let count_arr = document.get_array("totalCount")?;
	let count_0 = count_arr.get(0).ok_or("get count");
	let mut count = 0;
	match count_0 {
		Ok(c) => {
			count = c.as_document().ok_or("as document")?.get_i32("count")?;
		},
		Err(_) => {},
	}
	Ok(Some(GamePage {
		data: list_games,
		message: EMPTY_STR.to_string(),
		page: params.page,
		size: params.size,
		total: count as u64,
	}))
}
