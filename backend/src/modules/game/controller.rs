use actix_web::{
	get,
	http::StatusCode,
	post,
	web::{self, Data},
	Error as AWError, HttpResponse, Result,
};

use shared::constant::EMPTY_STR;

use super::dto::GameDTO;
use crate::{
	app_state::AppState,
	common::{Page, QueryPage, ResponseBody},
	modules::game::{
		dto::QueryFindGame,
		service::{find_game_by_id, find_games_by_query},
	},
};

#[utoipa::path(
    tag = "GameEndpoints",
    context_path = "/game",
    params((
		"game_id"=String,Path,description="ID of Game",example="Q29sbGVjdGlvblR5cGU6MjU5MzgzMjQ"
	)),
    responses(
        (status=200,description="Find Game Success",body=GameDTO),
        (status=NOT_FOUND,description="Can not found this game"))
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
			// Game not found, return 404 Not Found response
			Ok(HttpResponse::NotFound().finish())
		},
		Err(e) => {
			// Handle the error case, return 500 Internal Server Error response
			Ok(HttpResponse::InternalServerError().finish())
		},
	}
}

#[utoipa::path(
    post,
    tag = "GameEndpoints",
    context_path="/game",
    request_body(content =GameDTO,description="Request Body of Find Game of a address",content_type="application/json", example=json!({
		"search":"",
        "page": 1,
        "size": 10,
        "order_by": "create_at",
        "desc": true,
		 "query":{
			"owner":"0sxbdfc529688922fb5036d9439a7cd61d61114f700",
			"game_id":"",
			"category":"",
			"is_verified":true,

		 }
		
	})) , 
    responses(
        (status=StatusCode::OK,description="Find List Game Success",body=Vec<GameDTO>),
        (status=StatusCode::NOT_FOUND,description="Can not found List game"))
)]
#[post("/list")]
pub async fn get_games_by_address(
	app_state: Data<AppState>,
	req: web::Json<QueryPage<QueryFindGame>>,
) -> Result<HttpResponse, AWError> {
	let list_games = find_games_by_query(req.0, app_state.db.clone()).await;

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
	scope.service(get_games_by_address).service(get_game)
}
