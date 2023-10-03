use crate::{
	app_state::AppState,
	common::{QueryPage, ResponseBody, QueryNFT},
	modules::nft::{
			dto::{QueryFindNFts, NFTDTO},
			service::{find_nft_by_token, find_nfts_by_query, find_nfts_with_owner},
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
    get,
	operation_id="dsad",
    tag="NftEndpoints"
    ,path="/nft/{token_id}"
    ,params(
        ("token_id",Path,description="Token ID NFT",
        example="0xd774557b647330c91bf44cfeab205095f7e6c367"))
    ,responses(
        (status=200,description="Find NFT Success",body=NFTDTO),
        (status=NOT_FOUND,description="Cannot found this nft")
    )
)]
#[get("/{token_id}")]
pub async fn get_nft(
	app_state: Data<AppState>,
	path: web::Path<String>,
) -> Result<HttpResponse, AWError> {
	let token_id = path.into_inner();
	let nft_detail = find_nft_by_token(&token_id, app_state.db.clone()).await;
	match nft_detail {
		Ok(Some(nft)) => {
			let rsp = ResponseBody::<Option<NFTDTO>>::new(EMPTY_STR, Some(nft), true);
			Ok(HttpResponse::build(StatusCode::OK).content_type("application/json").json(rsp))
		},
		Ok(None) => {
			let rsp = ResponseBody::<Option<NFTDTO>>::new("Not found", None, false);
			Ok(HttpResponse::build(StatusCode::NOT_FOUND)
				.content_type("application/json")
				.json(rsp))
		},
		Err(e) => {
			let rsp = ResponseBody::<Option<NFTDTO>>::new(e.to_string().as_str(), None, false);
			Ok(HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(rsp))
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
        "order_by": "created_at",
        "desc": true,
        "query":
		{
			"address":"0sxbdfc529688922fb5036d9439a7cd61d61114f600",
			
		}
    })),
	
    responses(
        (status=StatusCode::OK,description="Find List NFTs Success",body=NFTWithOwnerPage),
        (status=StatusCode::INTERNAL_SERVER_ERROR,description="Error",body=NoData)

    )
)]
//Get List NFT Follow Address of Account
#[post("/list")]
pub async fn get_list_nft(
	app_state: Data<AppState>,
	req: web::Json<QueryNFT>,
) -> Result<HttpResponse, AWError> {
	let list_nft = find_nfts_with_owner(req.0, app_state.db.clone()).await;

	match list_nft {
		Ok(Some(nfts)) => {
			Ok(HttpResponse::build(StatusCode::OK).content_type("application/json").json(nfts))
		},
		Ok(None) => {
			let rsp =
				ResponseBody::<Option<NFTDTO>>::new("Not found", None, false);
			Ok(HttpResponse::build(StatusCode::NOT_FOUND)
				.content_type("application/json")
				.json(rsp))
		},
		Err(e) => {
			log::error!("{:?}",e);
			let rsp = ResponseBody::<Option<NFTDTO>>::new(e.to_string().as_str(), None, false);
			Ok(HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(rsp))
		},
	}
}

#[utoipa::path(
    post,
    tag = "NftEndpoints",
    context_path="/nft",
    request_body
	(content =QueryNFT,description=" Find NFTs By Search Query Data",content_type="application/json"
	,example=json!({
        "search":"",
        "page": 1,
        "size": 10,
        "order_by": "created_at",
        "desc": true,
        "query":
		{
			"name":null,
			"token_id":null,
			"collection_id":null
		}
    })),
    responses(
        (status=StatusCode::OK,description="Find List NFTs Success",body=NFTPage),
        (status=StatusCode::INTERNAL_SERVER_ERROR,description="Error",body=NoData)

    )
)]
#[post("/search")]
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
			let rsp =
				ResponseBody::<Option<NFTDTO>>::new("Not found", None, false);
			Ok(HttpResponse::build(StatusCode::NOT_FOUND)
				.content_type("application/json")
				.json(rsp))
		},
		Err(e) => {
			let rsp = ResponseBody::<Option<NFTDTO>>::new(e.to_string().as_str(), None, false);
			Ok(HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(rsp))
		},
	}
}

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
	scope.service(get_nft).service(get_list_nft).service(search_list_nfts)
}
