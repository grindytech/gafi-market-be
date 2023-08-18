use actix_web::{
	get,
	http::StatusCode,
	web::{self, Data},
	Error as AWError, HttpResponse,
};

use crate::{app_state::AppState, modules::collection::service::get_collection_by_id};
#[utoipa::path(
    tag="collection",
    context_path="/collection",
    params(("collection_id"=String,Path,description="Collection ID",example="Q29sbGVjdGlvblR5cGU6MjQxOTc3MTc")),
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
	let collection_detail = get_collection_by_id(&collection_id, app_state.db.clone()).await;
	match collection_detail {
		Ok(Some(collection_dto)) => Ok(HttpResponse::build(StatusCode::OK)
			.content_type("application/json")
			.json(collection_dto)),
		Ok(None) => Ok(HttpResponse::NotFound().finish()),
		Err(e) => {
			eprint!("Error From collection {:?}", e);
			Ok(HttpResponse::InternalServerError().finish())
		},
	}
}

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
	scope.service(get_collection)
}
