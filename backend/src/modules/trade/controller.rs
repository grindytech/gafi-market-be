use crate::{
	app_state::AppState,
	common::{QueryPage, ResponseBody},
};
use actix_web::{
	http::StatusCode,
	post,
	web::{self, Data},
	Error as AWError, HttpResponse, Result,
};

use super::{
	dto::{QueryFindTrade, TradeDTO},
	service::find_trades_by_query,
};
#[utoipa::path(
    post,
    tag = "TradeEndpoints",
    context_path="/trade",
    request_body
	(content =QueryFindTrade,description="Filter Trade By Query Data",content_type="application/json"
	,example=json!({
        "search":"",
        "page": 1,
        "size": 10,
        "order_by": "create_at",
        "desc": true,
        "query":
		{
			"trade_id":"",
            "trade_type":"",
		}
    })),
    responses(
        (status=StatusCode::OK,description="Find List NFTs Success",body=NFTPage),
        (status=StatusCode::INTERNAL_SERVER_ERROR,description="Error",body=NoData)

    )
)]
#[post("/search")]
pub async fn search_list_trade(
	app_state: Data<AppState>,
	req: web::Json<QueryPage<QueryFindTrade>>,
) -> Result<HttpResponse, AWError> {
	let list_trade = find_trades_by_query(req.0, app_state.db.clone()).await;
	match list_trade {
		Ok(Some(trade)) => {
			Ok(HttpResponse::build(StatusCode::OK).content_type("application/json").json(trade))
		},
		Ok(None) => {
			let rsp = ResponseBody::<Option<TradeDTO>>::new("Not found", None, false);
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
	scope.service(search_list_trade)
}
