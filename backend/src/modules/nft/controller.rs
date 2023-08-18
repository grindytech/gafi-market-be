use crate::{
	app_state::AppState,
	common::{Page, QueryPage, ResponseBody},
	modules::{
		game::dto::QueryInfo,
		nft::{
			dto::{QueryFindNFts, NFTDTO},
			service::{find_nfts, get_nft_by_token},
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
    tag="nft"
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
	let nft_detail = get_nft_by_token(&token_id, app_state.db.clone()).await;
	match nft_detail {
		Ok(Some(nft_dto)) => Ok(HttpResponse::build(StatusCode::OK)
			.content_type("application;.json")
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
    tag = "nft",
    context_path="/nft",
    request_body(content =QueryNFT,description="Request Body of find list NFTs by address",content_type="application/json",example=json!({
        "search":"",
        "page": 1,
        "size": 10,
        "order_by": "createdAt",
        "desc": true,
        "query":{"address":"0sxbdfc529688922fb5036d9439a7cd61d61114f600"}
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
	let list_nft = find_nfts(req.0, app_state.db.clone()).await;

	match list_nft {
		Ok(Some(nfts)) =>
			Ok(HttpResponse::build(StatusCode::OK).content_type("application/json").json(nfts)),
		Ok(None) => {
			let rsp: ResponseBody<Option<_>> =
				ResponseBody::<Option<()>>::new("Not found", None, false);
			Ok(HttpResponse::build(StatusCode::NOT_FOUND)
				.content_type("application/json")
				.json(rsp))
		},
		Err(e) => {
			let rsp = ResponseBody::<Option<()>>::new(e.to_string().as_str(), None, false);
			Ok(HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
				.content_type("application/json")
				.json(rsp))
		},
	}
}

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
	scope.service(get_nft).service(get_list_nft)
}