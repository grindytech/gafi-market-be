use actix_web::{
	get,
	http::StatusCode,
	post,
	web::{self, Data},
	Error as AWError, HttpResponse, Result,
};
use shared::constant::EMPTY_STR;

use crate::{
	app_state::AppState,
	common::{QueryPage, ResponseBody},
	modules::bundle::{
		dto::BundleDTO,
		service::{find_bundle_by_id, find_bundles_by_query},
	},
};

use super::dto::QueryFindBundles;

#[utoipa::path(
    get,
    tag="BundleEndpoints",
    context_path="/bundle",
    params(
        ("bundle_id"=String,Path,description="Bundle ID",
        example="lhtf7fbksg"
        )
    )
     ,responses(
        (status=OK,description="Find Bundle Detail Success",body=BundleDTO),
        (status=StatusCode::NOT_FOUND,description="Cannot found this Bundle")
    ),
)]
#[get("/{bundle_id}")]
pub async fn get_bundles(
	app_state: Data<AppState>,
	path: web::Path<String>,
) -> Result<HttpResponse, AWError> {
	let bundle_id = path.into_inner();

	let bundle_detail = find_bundle_by_id(&bundle_id, app_state.db.clone()).await;
	match bundle_detail {
		Ok(Some(bundle)) => {
			let rsp = ResponseBody::<Option<BundleDTO>>::new(EMPTY_STR, Some(bundle), true);
			Ok(HttpResponse::build(StatusCode::OK).content_type("application/json").json(rsp))
		},
		Ok(None) => {
			let rsp = ResponseBody::<Option<BundleDTO>>::new("Not found", None, false);
			Ok(HttpResponse::NotFound().content_type("application/json").json(rsp))
		},
		Err(e) => {
			let rsp = ResponseBody::<Option<()>>::new(e.to_string().as_str(), None, false);
			log::info!("Error {:?}", e);
			Ok(HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(rsp))
		},
	}
}
#[utoipa::path(
	post,
    tag="BundleEndpoints",
    context_path="/bundle",
    request_body(
		content=Querybundle,description="Find Collection by"
		,example=json!({
        "search":"",
        "page": 1,
        "size": 10,
        "order_by": "create_at",
        "desc": true,
        "query":
		{
			"name":"",
			"collection_id":"Q29sbGVjdGlvblR5cGU6MjQxOTc3MTc"
		}
    })),
    responses(
        (status= 200 , description="Search List Collections Success",body= NFTCollectionDTO),
        (status=NOT_FOUND,description="Collections was not found")
    )
)]
#[post("/search")]
pub async fn search_list_bundles(
	app_state: Data<AppState>,
	req: web::Json<QueryPage<QueryFindBundles>>,
) -> Result<HttpResponse, AWError> {
	let list_bundles = find_bundles_by_query(req.0, app_state.db.clone()).await;
	match list_bundles {
		Ok(Some(bundle)) => Ok(HttpResponse::build(StatusCode::OK)
			.content_type("application/json")
			.json(bundle)),
		Ok(None) => {
			let rsp: ResponseBody<Option<_>> =
				ResponseBody::<Option<()>>::new("Not found", None, false);
			Ok(HttpResponse::build(StatusCode::NOT_FOUND)
				.content_type("application/json")
				.json(rsp))
		},
		Err(e) => {
			let rsp = ResponseBody::<Option<()>>::new(e.to_string().as_str(), None, false);
			Ok(HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(rsp))
		},
	}
}
pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
	scope.service(get_bundles).service(search_list_bundles)
}
