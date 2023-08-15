use actix_web::{
    get,
    http::StatusCode,
    web::{self, Data},
    Error as AWError, HttpResponse, Result,
};

use crate::{app_state::AppState, modules::game::service::get_game_by_id};

#[utoipa::path(
    tag = "game",
    context_path = "/game",
    params(
        ("game_id"=String,Path,description="ID of Game",example="Q29sbGVjdGlvblR5cGU6MjU5MzgzMjQ")
    ),
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
    let game_detail = get_game_by_id(&game_id, app_state.db.clone()).await;
    match game_detail {
        Ok(Some(game_dto)) => Ok(HttpResponse::build(StatusCode::OK)
            .content_type("application/json")
            .json(game_dto)),
        Ok(None) => {
            // Game not found, return 404 Not Found response
            Ok(HttpResponse::NotFound().finish())
        }
        Err(e) => {
            // Handle the error case, return 500 Internal Server Error response
            eprintln!("Error: {:?}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

/* pub async fn get_games(app_state: Data<AppState>) -> Result<HttpResponse, AWError> {} */
pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
    scope.service(get_game)
}
