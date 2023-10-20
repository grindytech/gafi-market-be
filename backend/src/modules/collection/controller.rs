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
	common::{QueryCollection, ResponseBody},
	modules::collection::{
		dto::{NFTCollectionSupplyDTO, NFTCollectionVolumeDTO},
		service::{find_collection_supply_data, find_collection_volume_data, find_collections},
	},
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
			"games":["2"],
		}
    })),
    responses(
        (status= StatusCode::OK , description="Search List Collections Success",body= CollectionPage),
        (status=StatusCode::NOT_FOUND,description="Collections was not found")
    )
)]
/// Search collection by query
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

#[utoipa::path(
        tag = "CollectionEndpoints",
        context_path = "/collection",
        params((
			"collection_id"=String,Path,description="Collection ID of get Volume Data",example="0"
		)),
        responses(
            (status = StatusCode::OK, description = "Success Get Volume  Collection", body = NFTCollectionVolumeDTO),
			(status = StatusCode::NOT_FOUND, description = "Failed et Volume  Collection", body = NoData)
        ),
    )]
/// Get Volume a Collection Data (min_pice, max_price , volume, sold)
#[get("/volume_data/{collection_id}")]
pub async fn get_collection_volume_data(
	app_state: Data<AppState>,
	path: web::Path<String>,
) -> Result<HttpResponse, AWError> {
	let collection_id = path.into_inner();
	let result = find_collection_volume_data(collection_id, app_state.db.clone()).await;
	match result {
		Ok(Some(v)) =>
			Ok(HttpResponse::build(StatusCode::OK).content_type("application/json").json(v)),
		Ok(None) => {
			let rsp: ResponseBody<Option<NFTCollectionVolumeDTO>> =
				ResponseBody::<Option<NFTCollectionVolumeDTO>>::new("Invalid Data", None, false);
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

#[utoipa::path(
        tag = "CollectionEndpoints",
        context_path = "/collection",
        params((
			"collection_id"=String,Path,description="Collection ID of get Supply",example="0"
		)),
        responses(
            (status = StatusCode::OK, description = "Success Get Suppply Data Collection", body = NFTCollectionSupplyDTO),
			(status = StatusCode::NOT_FOUND, description = "Failed et Suppply Data Collection", body = NoData)
        ),
    )]
/// Get Collection Analysis Data Include Number owner ,total_supply
#[get("/supply_data/{collection_id}")]
pub async fn get_collection_supply_data(
	app_state: Data<AppState>,
	path: web::Path<String>,
) -> Result<HttpResponse, AWError> {
	let collection_id = path.into_inner();
	let result = find_collection_supply_data(collection_id, app_state.db.clone()).await;
	match result {
		Ok(Some(v)) =>
			Ok(HttpResponse::build(StatusCode::OK).content_type("application/json").json(v)),
		Ok(None) => {
			let rsp: ResponseBody<Option<NFTCollectionSupplyDTO>> =
				ResponseBody::<Option<NFTCollectionSupplyDTO>>::new("Invalid Data", None, false);
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
	scope
		.service(search_list_collections)
		.service(get_collection_volume_data)
		.service(get_collection_supply_data)
}
