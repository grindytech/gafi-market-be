use actix_web::{
	get,
	http::StatusCode,
	web::{self, Data},
	Error as AWError, HttpResponse,
};
use shared::constant::EMPTY_STR;

use crate::{
	app_state::{self, AppState},
	common::ResponseBody,
};

use super::{dto::TransactionDTO, service::find_tx_by_hash};

#[utoipa::path(
    tag = "TransactionEndpoints",
    context_path = "/tx",
    params((
		"tx_hash"=String,Path,description="ID of Game",example="Q29sbGVjdGlvblR5cGU6MjU5MzgzMjQ"
	)),
    responses(
        (status=200,description="Find Game Success",body=GameDTO),
        (status=NOT_FOUND,description="Can not found this game"))
)]
#[get("/{tx_hash}")]
pub async fn get_tx(
	app_state: Data<AppState>,
	path: web::Path<String>,
) -> Result<HttpResponse, AWError> {
	let tx_hash = path.into_inner();
	let tx_detail = find_tx_by_hash(&tx_hash, app_state.db.clone()).await;
	match tx_detail {
		Ok(Some(tx)) => {
			let rsp = ResponseBody::<Option<TransactionDTO>>::new(EMPTY_STR, Some(tx), true);
			Ok(HttpResponse::build(StatusCode::OK).content_type("application/json").json(rsp))
		},
		Ok(None) => {
			let rsp = ResponseBody::<Option<TransactionDTO>>::new("Not found", None, false);
			Ok(HttpResponse::build(StatusCode::NOT_FOUND)
				.content_type("application/json")
				.json(rsp))
		},
		Err(e) => {
			let rsp =
				ResponseBody::<Option<TransactionDTO>>::new(e.to_string().as_str(), None, false);
			Ok(HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(rsp))
		},
	}
}
pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
	scope.service(get_tx)
}
