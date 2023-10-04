use actix_web::{
	get,
	http::StatusCode,
	post,
	web::{self, Data},
	Error as AWError, HttpResponse, Result,
};

use super::dto::GameDTO;

use crate::{
	app_state::AppState,
	common::{QueryGame, QueryPage, ResponseBody},
	modules::game::{
		dto::QueryFindGame,
		service::{find_game_by_id, find_games_by_query},
	},
};
use shared::constant::EMPTY_STR;

#[utoipa::path(
    tag = "GameEndpoints",
    context_path = "/game",
    params((
		"game_id"=String,Path,description="ID of Game",example="0"
	)),
    responses(
        (status=200,description="Find Game Detail Success",body=GameDTO),
        (status=NOT_FOUND,description="Can Not Found This Game"))
)]
#[get("/{game_id}")]
pub async fn get_game(
	app_state: Data<AppState>,
	path: web::Path<String>,
) -> Result<HttpResponse, AWError> {
	let game_id = path.into_inner();
	let game_detail = find_game_by_id(&game_id, app_state.db.clone()).await;
	match game_detail {
		Ok(Some(game)) => {
			let rsp = ResponseBody::<Option<GameDTO>>::new(EMPTY_STR, Some(game), true);
			Ok(HttpResponse::build(StatusCode::OK).content_type("application/json").json(rsp))
		},
		Ok(None) => {
			let rsp = ResponseBody::<Option<GameDTO>>::new("Not found", None, false);
			Ok(HttpResponse::build(StatusCode::NOT_FOUND)
				.content_type("application/json")
				.json(rsp))
		},
		Err(e) => {
			let rsp = ResponseBody::<Option<GameDTO>>::new(e.to_string().as_str(), None, false);
			Ok(HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(rsp))
		},
	}
}

#[utoipa::path(
	post,
    tag = "GameEndpoints",
    context_path="/game",
    request_body(
		content=QueryGame,description="Find Collection by"
		,example=json!({
        "search":"",
        "page": 1,
        "size": 10,
        "order_by": "created_at",
        "desc": true,
        "query":
		{
			"game_id":null,
			"owner":null,
			"category":null,
			"is_verfied":null,
		}
    })),
    responses(
        (status=StatusCode::OK,description="Find List Game Success",body=GamePage),
        (status=StatusCode::NOT_FOUND,description="Can not found List game"))
)]
#[post("/search")]

pub async fn search_games_by_query(
	app_state: Data<AppState>,
	path: web::Json<QueryGame>,
) -> Result<HttpResponse, AWError> {
	let list_games = find_games_by_query(path.0, app_state.db.clone()).await;
	match list_games {
		Ok(Some(games)) => {
			Ok(HttpResponse::build(StatusCode::OK).content_type("application/json").json(games))
		},
		Ok(None) => {
			let rsp = ResponseBody::<Option<GameDTO>>::new("Game Not found", None, false);
			Ok(HttpResponse::build(StatusCode::NOT_FOUND)
				.content_type("application/json")
				.json(rsp))
		},
		Err(e) => {
			let rsp = ResponseBody::<Option<()>>::new(e.to_string().as_str(), None, false);
			Ok(HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(rsp))
		},
	}
}

/* pub async fn get_games(app_state: Data<AppState>) -> Result<HttpResponse, AWError> {} */
pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
	scope.service(search_games_by_query).service(get_game)
}
