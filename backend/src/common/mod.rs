pub mod utils;

use serde::{Deserialize, Serialize};
use utoipa::{
	openapi::{Object, ObjectBuilder},
	PartialSchema, ToSchema,
};

use crate::modules::{
	account::dto::AccountDTO,
	collection::dto::{QueryFindCollections, NFTCollectionDTO},
	game::dto::{QueryFindGame, GameDTO},
	nft::dto::{QueryFindNFts, NFTDTO}, transaction::dto::{QueryFindTX, HistoryTxDTO}, pool::dto::{QueryFindPool, PoolDTO},
};

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
	pub status: String,
	pub message: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[aliases(AccountObject = ResponseBody<AccountDTO>, NoData = ResponseBody<NoResponse>)]
pub struct ResponseBody<T> {
	pub success: bool,
	pub message: String,
	pub data: T,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct NoResponse {}

impl<T> ResponseBody<T> {
	pub fn new(message: &str, data: T, success: bool) -> ResponseBody<T> {
		ResponseBody {
			message: message.to_string(),
			data,
			success,
		}
	}
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[aliases(
	NFTPage = Page<NFTDTO>,
	GamePage=Page<GameDTO>,
	TxPage = Page<HistoryTxDTO>,
	CollectionPage = Page<NFTCollectionDTO>,
	PoolPage = Page<PoolDTO>,
)]
pub struct Page<T> {
	pub message: String,
	pub data: Vec<T>,
	pub page: u64,
	pub size: u64,
	pub total: u64,
}

impl<T> Page<T> {
	pub fn new(message: &str, data: Vec<T>, page: u64, size: u64, total: u64) -> Page<T> {
		Page {
			message: message.to_string(),
			data,
			page,
			size,
			total,
		}
	}
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[aliases(
	QueryNFT = QueryPage<QueryFindNFts>, 
	QueryCollection = QueryPage<QueryFindCollections> ,
	QueryGame=QueryPage<QueryFindGame>,
	QueryTransaction=QueryPage<QueryFindTX>,
	QueryPool=QueryPage<QueryFindPool>
)]

pub struct QueryPage<T> {
	pub search: String,
	pub page: u64,
	pub size: u64,
	pub order_by: String,
	pub desc: bool,
	pub query: T,
}

#[derive(Debug, Serialize, Deserialize)]
struct TokenPayload {
	address: String,
	/* sub: String, */
	iat: i64,
	exp: i64,
}

mod types;
pub use types::*;
