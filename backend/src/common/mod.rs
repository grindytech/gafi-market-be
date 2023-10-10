pub mod utils;

use core::fmt;

use mongodb::bson::Document;
use serde::{Deserialize, Serialize};
use utoipa::{
	openapi::{Object, ObjectBuilder},
	PartialSchema, ToSchema,
};

use crate::modules::{
	account::dto::{AccountDTO, QueryFindAccount},
	collection::dto::{QueryFindCollections, NFTCollectionDTO},
	game::dto::{QueryFindGame, GameDTO},
	nft::dto::{QueryFindNFts, NFTDTO,NFTOwnerOfDto}, transaction::dto::{QueryFindTX, HistoryTxDTO}, pool::dto::{QueryFindPool, PoolDTO}, auth::dto::TokenDTO,
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
#[aliases( NoData = ResponseBody<NoResponse>,AccountData = ResponseBody<AccountDTO>,TokenData=ResponseBody<TokenDTO>)]
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
	NFTWithOwnerPage = Page<NFTOwnerOfDto>,
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

impl <T> QueryPage<T> {
	pub fn skip(&self) -> i64 {
		let skip = (self.page -1) * self.size;
		skip.to_string().parse::<i64>().unwrap_or(10)
	}
	pub fn size(&self) -> i64 {
		if self.size > 100 {
			return 100
		}
		self.size.to_string().parse().unwrap_or(10)
	}
	pub fn sort(&self) -> Document {
		let mut doc =Document::new();
		let mut desc = -1;
		if !self.desc {
			desc = 1;
		}
		doc.insert(self.order_by.clone(), desc);
		doc
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPayload {
	pub address: String,
	/* sub: String, */
	pub iat: i64,
	pub exp: i64,
}
pub const JWT_ACCESS_TIME:i64=3600;
pub const JWT_REFRESH_TIME:i64=86400;


mod types;
pub use types::*;
