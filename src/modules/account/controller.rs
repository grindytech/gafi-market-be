use crate::{app_state::AppState, modules::account::service::get_account_by_adress};
use actix_web::{
    get,
    http::StatusCode,
    web::{self, Data},
    Error as AWError, HttpResponse, Result,
};

#[utoipa::path(
        tag = "account",
        context_path = "/account",
        params(("data_id"=String,Path,description="ID of account",example="0sxbdfc529688922fb5036d9439a7cd61d61114f600")),
        responses(
            (status = 200, description = "Account Find Success", body = AccountDTO),
            (status = NOT_FOUND, description = "Account was not found")
        ),
    )]
#[get("/{data_id}")]
pub async fn get_account(
    app_state: Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse, AWError> {
    let data_id = path.into_inner();
    let account_detail = get_account_by_adress(&data_id, app_state.db.clone()).await;
    match account_detail {
        Ok(Some(account_dto)) => {
            // Convert AccountDTO to JSON and build the HTTP response

            Ok(HttpResponse::build(StatusCode::OK)
                .content_type("application/json")
                .json(account_dto))
        }
        Ok(None) => {
            // Account not found, return 404 Not Found response
            Ok(HttpResponse::NotFound().finish())
        }
        Err(e) => {
            // Handle the error case, return 500 Internal Server Error response
            eprintln!("Error: {:?}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}
/// returns the endpoints for the Auth service
pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
    scope.service(get_account)
}
