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
	common::{ ResponseBody, QueryTx},
	modules::transaction::service::find_tx_by_query,
};

use super::dto:: HistoryTxDTO;

#[utoipa::path(
	post,
	tag="HistoryTransactionEndpoints",
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
			"collection_id":null,
			"token_id":null,
			"trade_id":null,
			"event":null,
			"address":null,
			"pool_id":null
		}
    })
		
	)
)]

//Search Transaction By Query
#[post("/search")]
pub async fn search_history_tx(
	app_state: Data<AppState>,
	req: web::Json<QueryTx>,
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
	scope.service(search_history_tx)
}
