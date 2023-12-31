use actix_web::{
	http::StatusCode,
	post,
	web::{self, Data},
	Error as AWError, HttpResponse, Result,
};

use super::dto::GameDTO;

use crate::{
	app_state::AppState,
	common::{QueryGame, ResponseBody},
	modules::game::service::find_games_by_query,
};
use shared::constant::EMPTY_STR;
/// Search Game From Database By Query
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
        "order_by": "updated_at",
        "desc": true,
        "query":
		{
			"game_id":null,
			"owner":null,
			"collection":null,
			"name":null,
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
		Ok(Some(games)) =>
			Ok(HttpResponse::build(StatusCode::OK).content_type("application/json").json(games)),
		Ok(None) => {
			let rsp = ResponseBody::<Option<GameDTO>>::new("Not found", None, false);
			Ok(HttpResponse::build(StatusCode::NOT_FOUND)
				.content_type("application/json")
				.json(rsp))
		},
		Err(e) => {
			log::info!("Error Game Server {:?}", e.to_string());
			let rsp = ResponseBody::<Option<()>>::new(EMPTY_STR, None, false);

			Ok(HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(rsp))
		},
	}
}

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
	scope.service(search_games_by_query)
}
