use actix_web::{
	http::StatusCode,
	post,
	web::{self, Data},
	Error as AWError, HttpResponse,
};

use crate::{
	app_state::AppState,
	common::{QueryPool, ResponseBody},
	modules::pool::service::find_pool_by_query,
};

use super::dto::PoolDTO;

#[utoipa::path(
    post,
    tag="PoolEndpoints",
    context_path="/pool",
    request_body(
        content=QueryPool,description="Search Pool By Query Data", content_type="application/json",
        example=json!({
            "search":"",
        "page": 1,
        "size": 10,
        "order_by": "created_at",
        "desc": true,
        "query":{
            "pool_id":null,
            "owner":null,
            "type_pool":null,
            "admin":null,
            "owner_deposit":null
        }
        })
    ),
    responses(
        (status=StatusCode::OK , description = " Search List Pools Success" , body=PoolPage),
        (status=StatusCode::INTERNAL_SERVER_ERROR,description="Error",body=NoData)
    )
)]
#[post("/search")]
pub async fn search_list_pools(
	app_state: Data<AppState>,
	req: web::Json<QueryPool>,
) -> Result<HttpResponse, AWError> {
	let list_pool = find_pool_by_query(req.0, app_state.db.clone()).await;

	match list_pool {
		Ok(Some(pool)) => {
			Ok(HttpResponse::build(StatusCode::OK).content_type("application/json").json(pool))
		},
		Ok(None) => {
			let rsp = ResponseBody::<Option<PoolDTO>>::new("Not found", None, false);
			Ok(HttpResponse::build(StatusCode::NOT_FOUND)
				.content_type("application/json")
				.json(rsp))
		},
		Err(e) => {
			let rsp = ResponseBody::<Option<PoolDTO>>::new(e.to_string().as_str(), None, false);
			Ok(HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(rsp))
		},
	}
}
pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
	scope.service(search_list_pools)
}
