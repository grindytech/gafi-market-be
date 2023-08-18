use crate::{
	app_state::AppState,
	common::{Page, QueryPage, ResponseBody},
	modules::{
		game::dto::QueryInfo,
		nft::{
			dto::{QueryFindNFts, NFTDTO},
			service::{find_nft_by_token, find_nfts_by_address, find_nfts_by_query},
		},
	},
};
use actix_web::{
	get,
	http::StatusCode,
	post,
	web::{self, Data},
	Error as AWError, HttpResponse, Result,
};
use serde::{Deserialize, Serialize};

#[utoipa::path(
    get,
    tag="NftEndpoints"
    ,path="/nft/{token_id}"
    ,params(
        ("token_id",Path,description="Token ID NFT",
        example="0xd774557b647330c91bf44cfeab205095f7e6c367"))
    ,responses(
        (status=200,description="Find NFT Success",body=NFTDTO),
        (status=NOT_FOUND,description="Cannot found this nft")
    ))]
#[get("/{token_id}")]
pub async fn get_nft(
	app_state: Data<AppState>,
	path: web::Path<String>,
) -> Result<HttpResponse, AWError> {
	let token_id = path.into_inner();
	let nft_detail = find_nft_by_token(&token_id, app_state.db.clone()).await;
	match nft_detail {
		Ok(Some(nft_dto)) => Ok(HttpResponse::build(StatusCode::OK)
			.content_type("application/json")
			.json(nft_dto)),
		Ok(None) => {
			// NFT not found, return 404 Not Found response
			Ok(HttpResponse::NotFound().finish())
		},
		Err(e) => {
			// Handle the error case, return 500 Internal Server Error response
			eprintln!("Error: {:?}", e);
			Ok(HttpResponse::InternalServerError().finish())
		},
	}
}

#[utoipa::path(
    post,
    tag = "NftEndpoints",
    context_path="/nft",
    request_body(content =QueryNFT,description="Request Body of find list NFTs by address",content_type="application/json",example=json!({
        "search":"",
        "page": 1,
        "size": 10,
        "order_by": "createdAt",
        "desc": true,
        "query":{"address":"0sxbdfc529688922fb5036d9439a7cd61d61114f600","name":""}
    })),
    responses(
        (status=StatusCode::OK,description="Find List NFTs Success",body=NFTPage),
        (status=StatusCode::INTERNAL_SERVER_ERROR,description="Error",body=NoData)

    )
)]
//Get List NFT Follow Address
#[post("/list")]
pub async fn get_list_nft(
	app_state: Data<AppState>,
	req: web::Json<QueryPage<QueryFindNFts>>,
) -> Result<HttpResponse, AWError> {
	let list_nft = find_nfts_by_address(req.0, app_state.db.clone()).await;

	match list_nft {
		Ok(Some(nfts)) => {
			Ok(HttpResponse::build(StatusCode::OK).content_type("application/json").json(nfts))
		},
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

#[utoipa::path(
    post,
    tag = "NftEndpoints",
    context_path="/nft",
    request_body(content =QueryNFT,description=" Search Query",content_type="application/json",example=json!({
        "search":"",
        "page": 1,
        "size": 10,
        "order_by": "createdAt",
        "desc": true,
        "query":{"name":"#189337","token_id":"0xd774557b647330c91bf44cfeab205095f7e6c368"}
    })),
    responses(
        (status=StatusCode::OK,description="Find List NFTs Success",body=NFTPage),
        (status=StatusCode::INTERNAL_SERVER_ERROR,description="Error",body=NoData)

    )
)]
#[post("/test")]
pub async fn search_list_nfts(
	app_state: Data<AppState>,
	req: web::Json<QueryPage<QueryFindNFts>>,
) -> Result<HttpResponse, AWError> {
	let list_nfts = find_nfts_by_query(req.0, app_state.db.clone()).await;
	match list_nfts {
		Ok(Some(nfts)) => {
			Ok(HttpResponse::build(StatusCode::OK).content_type("application/json").json(nfts))
		},
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
	scope.service(get_nft).service(get_list_nft).service(search_list_nfts)
}
