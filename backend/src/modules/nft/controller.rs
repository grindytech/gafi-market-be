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
			"collection_id":null,
			"token_id":"0",
			"address":null,

			
		}
    })),
	
    responses(
        (status=StatusCode::OK,description="Find List NFTs Success",body=NFTWithOwnerPage),
        (status=StatusCode::INTERNAL_SERVER_ERROR,description="Error",body=NoData)

    )
)]
//Get List NFT Follow Address of Account
#[post("/owner")]
pub async fn get_owner_nfts(
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
			"collection_id":null,
			"created_by":null,
			"attributes":[{"key":"tier","value":"\"King\""},{"key":"elo","value":"2700"}]
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
			log::info!("Error {:?}",e.to_string());
			Ok(HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(rsp))
		},
	}
}

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
	scope.service(get_owner_nfts).service(search_list_nfts)
}
