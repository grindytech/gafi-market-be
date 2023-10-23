use crate::{
	app_state::{self, AppState},
	common::{QueryTrade, ResponseBody},
	modules::trade::{dto::TradeDTO, service::find_trade_by_query},
};
use actix_web::{
	http::StatusCode,
	post,
	web::{self, Data},
	Error as AWError, HttpResponse,
};

#[utoipa::path(
	post,
	tag="TradeEndpoints",
	context_path="/trade",
	request_body(
		content=QueryTrade,description="Search Trade By Query Data", content_type="application/json",
		example=json!({
        "search":"",
		"page": 1,
		"size": 10,
		"order_by": "trade_type",
		"desc": true,
		"query":{
			"trade_id":null,		
		}
		})
	),
	responses(
		(status=StatusCode::OK , description = " Search List Pools Success" , body=TradePage),
		(status=StatusCode::INTERNAL_SERVER_ERROR,description="Error",body=NoData)
	)
)]
#[post("/search")]
pub async fn search_list_trades(
	app_state: Data<AppState>,
	req: web::Json<QueryTrade>,
) -> Result<HttpResponse, AWError> {
	let list_trade = find_trade_by_query(req.0, app_state.db.clone()).await;
	match list_trade {
		Ok(Some(trade)) =>
			Ok(HttpResponse::build(StatusCode::OK).content_type("application/json").json(trade)),
		Ok(None) => {
			let rsp = ResponseBody::<Option<TradeDTO>>::new("Trade Not found", None, false);
			Ok(HttpResponse::build(StatusCode::NOT_FOUND)
				.content_type("application/json")
				.json(rsp))
		},
		Err(e) => {
			let rsp = ResponseBody::<Option<TradeDTO>>::new(e.to_string().as_str(), None, false);
			Ok(HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(rsp))
		},
	}
}
pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
	scope.service(search_list_trades)
}
