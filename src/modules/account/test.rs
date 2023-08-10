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
                .json({ account_dto }))
        }
        Ok(None) => {
            // Account not found, return 404 Not Found response
            Ok(HttpResponse::NotFound().finish())
        }
        Err(e) => {
            // Handle the error case, return 500 Internal Server Error response
            eprintln!("Error: {:?}", e);
            println!("Error from Response");
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

async fn test() {}
