use crate::{
	app_state::{self, AppState},
	common::ResponseBody,
	modules::categories::{
		dto::{CategoriesDTO, QueryCategory},
		service::{create_category, get_categories},
	},
};
use actix_web::{
	get,
	http::StatusCode,
	post,
	web::{self, Data},
	Error as AWError, HttpResponse, Result,
};
use shared::constant::EMPTY_STR;
#[utoipa::path(
    tag="CategoriesEndpoints",
    context_path="/categories",

    responses(
        (status= StatusCode::OK , description="List Categories Success",body= Vec<CategoriesDTO>),
        (status=StatusCode::NOT_FOUND,description="List Categories was not found")
    )

)]
#[get("/list")]
pub async fn get_list_categories(app_state: Data<AppState>) -> Result<HttpResponse, AWError> {
	let list_categories = get_categories(app_state.db.clone()).await;
	match list_categories {
		Ok(Some(categories)) => {
			let rsp = ResponseBody::<Vec<CategoriesDTO>>::new(EMPTY_STR, categories, true);
			Ok(HttpResponse::build(StatusCode::OK).content_type("application/json").json(rsp))
		},
		Ok(None) => {
			let rsp = ResponseBody::<Option<CategoriesDTO>>::new("Not found", None, false);
			Ok(HttpResponse::build(StatusCode::NOT_FOUND)
				.content_type("application/json")
				.json(rsp))
		},
		Err(e) => {
			let rsp =
				ResponseBody::<Option<CategoriesDTO>>::new(e.to_string().as_str(), None, false);
			Ok(HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(rsp))
		},
	}
}

#[utoipa::path(
	post,
	tag="CategoriesEndpoints",
	context_path="/categories",
	request_body(
		content=QueryCategory,description="Create New Categories",
		example=json!({
			"name":"",
		})
	),
	responses(
        (status=StatusCode::OK,description="Create Category Success",body=CategoriesDTO),
        (status=StatusCode::INTERNAL_SERVER_ERROR,description="Error",body=NoData)

   	 )
)]
#[post("/create")]
pub async fn create_new_category(
	app_state: Data<AppState>,
	req: web::Json<QueryCategory>,
) -> Result<HttpResponse, AWError> {
	let result = create_category(req.0.name, app_state.db.clone()).await;
	match result {
		Ok(Some(categories)) => {
			let categories_dto: CategoriesDTO = categories.into();
			Ok(HttpResponse::build(StatusCode::OK)
				.content_type("application/json")
				.json(categories_dto))
		},
		Ok(None) => {
			let rsp = ResponseBody::<Option<()>>::new("Category Already Exist", None, false);
			Ok(HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(rsp))
		},
		Err(e) => {
			let rsp = ResponseBody::<Option<()>>::new(e.to_string().as_str(), None, false);
			Ok(HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(rsp))
		},
	}
}

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
	scope.service(get_list_categories).service(create_new_category)
}
