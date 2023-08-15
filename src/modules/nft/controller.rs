use crate::{app_state::AppState, modules::nft::service::get_nft_by_token};
use actix_web::{
    get,
    http::StatusCode,
    web::{self, Data},
    Error as AWError, HttpResponse, Result,
};

#[utoipa::path(
    get,
    tag="nft"
    ,path="/{token_id}"
    ,params(
        ("token_id",Path,description="Token ID NFT",
        example="0xd774557b647330c91bf44cfeab205095f7e6c367"))
    ,responses(
        (status=200,description="Find NFT Success",body=NftDTO),
        (status=NOT_FOUND,description="Cannot found this nft")
    ))]
#[get("/{token_id}")]
pub async fn get_nft(
    app_state: Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse, AWError> {
    let token_id = path.into_inner();
    let nft_detail = get_nft_by_token(&token_id, app_state.db.clone()).await;
    match nft_detail {
        Ok(Some(nft_dto)) => Ok(HttpResponse::build(StatusCode::OK)
            .content_type("application;.json")
            .json(nft_dto)),
        Ok(None) => {
            // NFT not found, return 404 Not Found response
            Ok(HttpResponse::NotFound().finish())
        }
        Err(e) => {
            // Handle the error case, return 500 Internal Server Error response
            eprintln!("Error: {:?}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
    scope.service(get_nft)
}
