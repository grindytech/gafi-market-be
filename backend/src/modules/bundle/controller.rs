use actix_web::{
	get,
	http::StatusCode,
	web::{self, Data},
	Error as AWError, HttpResponse, Result,
};
use shared::constant::EMPTY_STR;

use crate::{
	app_state::AppState,
	common::ResponseBody,
	modules::bundle::{dto::BundleDTO, service::find_bundle_by_id},
};

#[utoipa::path(
    get,
    tag="BundleEnpoints",
    context_path="/bundle",
    params(
        ("bundle_id"=String,Path,description="Bundle ID",
        example="lhtf7fbksg"
        )
    )
     ,responses(
        (status=OK,description="Find Bundle Success",body=BundleDTO),
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
			Ok(HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(rsp))
		},
	}
}

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
	scope.service(get_bundles)
}
