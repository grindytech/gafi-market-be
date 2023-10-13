use actix_web::{
	http::StatusCode,
	post,
	web::{self, Data},
	Error as AWError, HttpResponse,
};
use shared::constant::EMPTY_STR;

use crate::{
	app_state::AppState,
	common::{QueryCollection, ResponseBody},
	modules::collection::service::find_collections,
};

#[utoipa::path(
	post,
    tag="CollectionEndpoints",
    context_path="/collection",
    request_body(
		content=QueryCollection,description="Find Collection by"
		,example=json!({
        "search":"",
        "page": 1,
        "size": 10,
        "order_by": "created_at",
        "desc": true,
        "query":
		{
			"name":null,
			"owner":null,
			"collection_id":null,
			"game_id":["2"],
		}
    })),
    responses(
        (status= StatusCode::OK , description="Search List Collections Success",body= CollectionPage),
        (status=StatusCode::NOT_FOUND,description="Collections was not found")
    )
)]
#[post("/search")]
pub async fn search_list_collections(
	app_state: Data<AppState>,
	req: web::Json<QueryCollection>,
) -> Result<HttpResponse, AWError> {
	let list_collections = find_collections(req.0, app_state.db.clone()).await;
	/* log::info!("Error {:?}",list_collections); */
	match list_collections {
		Ok(Some(collections)) => Ok(HttpResponse::build(StatusCode::OK)
			.content_type("application/json")
			.json(collections)),
		Ok(None) => {
			let rsp: ResponseBody<Option<_>> =
				ResponseBody::<Option<()>>::new("Not found", None, false);
			Ok(HttpResponse::build(StatusCode::NOT_FOUND)
				.content_type("application/json")
				.json(rsp))
		},
		Err(e) => {
			log::info!("Error Collection Server {:?}", e.to_string());
			let rsp = ResponseBody::<Option<()>>::new(EMPTY_STR, None, false);
			Ok(HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(rsp))
		},
	}
}

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
	scope.service(search_list_collections)
}
