use super::service;

use crate::{
    app_state::AppState,
    modules::account::{dto::AccountDTO, service::get_account},
    utils::db,
};
use actix_web::{
    get,
    http::StatusCode,
    post,
    web::{self, Data, Json, Path},
    Error as AWError, HttpResponse, Responder, Result,
};
use serde_json::json;
use utoipa::OpenApi;
#[utoipa::path(
        get,
        path = "/{data_id}",
        params(("data_id"=String,Path,description="ID of account",example="0sxbdfc529688922fb5036d9439a7cd61d61114f600")),
        responses(
            (status = 200, description = "Account Find Success", body = AccountDTO,example=json!(AccountDTO{address:String::from("0sxbdfc529688922fb5036d9439a7cd61d61114f600"), balance: "200.00".to_string(),
                is_verified: true,
                name: "Who more".to_string(),
                bio: "A simple description.".to_string(),
                logo_url:Some( "https://example.com/logo.png".to_string()),
                banner_url: Some("https://example.com/banner.png".to_string())})),
               
            (status = NOT_FOUND, description = "Account was not found")
        ),
      
      
    )]
#[get("/{data_id}")]
pub async fn get_define_account(
    app_state: Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse, AWError> {
    let data_id = path.into_inner();
    let account_detail = get_account(&data_id, app_state.db.clone()).await;
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
    scope.service(get_define_account)
}

#[derive(OpenApi)]
#[openapi(
    paths(get_define_account),
    components(
        schemas(
            AccountDTO,
        )
    ),
    tags(
        (name = "account", description = "account API"),
    ),
    servers(
        (url = "/account")
    )
)]

pub struct ApiDoc;
