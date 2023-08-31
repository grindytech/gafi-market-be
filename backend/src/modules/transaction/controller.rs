use actix_web::{
	get,
	http::StatusCode,
	post,
	web::{self, Data},
	Error as AWError, HttpResponse,
};
use shared::constant::EMPTY_STR;

use crate::{
	app_state::AppState,
	common::{QueryPage, ResponseBody},
	modules::transaction::service::find_tx_by_query,
};

use super::{
	dto::{QueryFindTX, HistoryTxDTO},
	service::find_tx_by_hash,
};

#[utoipa::path(
    tag = "TransactionEndpoints",
    context_path = "/tx",
    params((
		"tx_hash"=String,Path,description="ID of Transaction",example="Q29sbGVjdGlvblR5cGU6MjU5MzgzMjQ"
	)),
    responses(
        (status=200,description="Find Transaction Success",body=HistoryTxDTO),
        (status=NOT_FOUND,description="Can not found this TX"))
)]
#[get("/{tx_hash}")]
pub async fn get_history_tx(
	app_state: Data<AppState>,
	path: web::Path<String>,
) -> Result<HttpResponse, AWError> {
	let tx_hash = path.into_inner();
	let tx_detail = find_tx_by_hash(&tx_hash, app_state.db.clone()).await;
	match tx_detail {
		Ok(Some(tx)) => {
			let rsp = ResponseBody::<Option<HistoryTxDTO>>::new(EMPTY_STR, Some(tx), true);
			Ok(HttpResponse::build(StatusCode::OK).content_type("application/json").json(rsp))
		},
		Ok(None) => {
			let rsp = ResponseBody::<Option<HistoryTxDTO>>::new("Not found", None, false);
			Ok(HttpResponse::build(StatusCode::NOT_FOUND)
				.content_type("application/json")
				.json(rsp))
		},
		Err(e) => {
			let rsp =
				ResponseBody::<Option<HistoryTxDTO>>::new(e.to_string().as_str(), None, false);
			Ok(HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(rsp))
		},
	}
}
#[utoipa::path(
	post,
	tag="TransactionEndpoints",
	context_path="/tx",
	request_body
	(
		content=QueryTransaction,
		description="Find Transactions By Search Query Data",
		content_type="application/json",
		example=json!({
        "search":"",
        "page": 1,
        "size": 10,
        "order_by": "create_at",
        "desc": true,
        "query":
		{
			"tx_hash":null,
			"from":null,
			"to":null,
			"game_id":null,
			"token_id":null,
			"collection_id":null
		}
    })
		
	)
)]

//Search Transaction By Query
#[post("/search")]
pub async fn search_history_tx(
	app_state: Data<AppState>,
	req: web::Json<QueryPage<QueryFindTX>>,
) -> Result<HttpResponse, AWError> {
	let transactions = find_tx_by_query(req.0, app_state.db.clone()).await;
	match transactions {
		Ok(Some(tx)) => {
			Ok(HttpResponse::build(StatusCode::OK).content_type("application/json").json(tx))
		},
		Ok(None) => {
			let rsp = ResponseBody::<Option<HistoryTxDTO>>::new("Not found", None, false);
			Ok(HttpResponse::build(StatusCode::NOT_FOUND)
				.content_type("application/json")
				.json(rsp))
		},
		Err(e) => {
			let rsp =
				ResponseBody::<Option<HistoryTxDTO>>::new(e.to_string().as_str(), None, false);
			Ok(HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(rsp))
		},
	}
}
pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
	scope.service(get_history_tx).service(search_history_tx)
}
