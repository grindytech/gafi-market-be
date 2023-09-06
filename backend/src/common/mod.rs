pub mod utils;

use core::fmt;

use serde::{Deserialize, Serialize};
use utoipa::{
	openapi::{Object, ObjectBuilder},
	PartialSchema, ToSchema,
};

use crate::modules::{
	account::dto::{AccountDTO, QueryFindAccount},
	collection::dto::{QueryFindCollections, NFTCollectionDTO},
	game::dto::{QueryFindGame, GameDTO},
	nft::dto::{QueryFindNFts, NFTDTO}, transaction::dto::{QueryFindTX, HistoryTxDTO}, pool::dto::{QueryFindPool, PoolDTO},
};

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
	pub status: String,
	pub message: String,
}
impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[aliases( NoData = ResponseBody<NoResponse>,AccountData = ResponseBody<AccountDTO>)]
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
	QueryPool=QueryPage<QueryFindPool>,
	QueryAccount=QueryPage<QueryFindAccount>
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
pub struct TokenPayload {
	pub address: String,
	/* sub: String, */
	pub iat: i64,
	pub exp: i64,
}

mod types;
pub use types::*;
