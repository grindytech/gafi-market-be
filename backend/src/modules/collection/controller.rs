use actix_web::{
	get,
	http::StatusCode,
	web::{self, Data},
	Error as AWError, HttpResponse,
};
use shared::constant::EMPTY_STR;

use crate::{
	app_state::AppState,
	common::ResponseBody,
	modules::collection::{dto::NFTCollectionDTO, service::find_collection_by_id},
};
#[utoipa::path(
    tag="CollectionEndpoints",
    context_path="/collection",
    params((
		"collection_id"=String,Path,description="Collection ID",example="Q29sbGVjdGlvblR5cGU6MjQxOTc3MTc"
	)),
    responses(
        (status= 200 , description="Find Collection Success",body= NFTCollectionDTO),
        (status=NOT_FOUND,description="Collection was not found")
    )
)]
#[get("/{collection_id}")]
pub async fn get_collection(
	app_state: Data<AppState>,
	path: web::Path<String>,
) -> Result<HttpResponse, AWError> {
	let collection_id = path.into_inner();
	let collection_detail = find_collection_by_id(&collection_id, app_state.db.clone()).await;
	match collection_detail {
		Ok(Some(collection_dto)) => {
			let rsp = ResponseBody::<Option<NFTCollectionDTO>>::new(
				EMPTY_STR,
				Some(collection_dto),
				true,
			);
			Ok(HttpResponse::build(StatusCode::OK).content_type("application/json").json(rsp))
		},
		Ok(None) => {
			let rsp = ResponseBody::<Option<NFTCollectionDTO>>::new("Not found", None, false);
			Ok(HttpResponse::build(StatusCode::NOT_FOUND)
				.content_type("application/json")
				.json(rsp))
		},
		Err(e) => {
			let rsp =
				ResponseBody::<Option<NFTCollectionDTO>>::new(e.to_string().as_str(), None, false);
			Ok(HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(rsp))
		},
	}
}

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
	scope.service(get_collection)
}
