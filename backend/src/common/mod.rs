use serde::{Deserialize, Serialize};
use utoipa::{
	openapi::{Object, ObjectBuilder},
	PartialSchema, ToSchema,
};

use crate::modules::{
	account::dto::AccountDTO,
	nft::dto::{QueryFindNFts, NFTDTO},
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
#[aliases(NFTPage = Page<NFTDTO>)]
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
#[aliases(QueryNFT = QueryPage<QueryFindNFts>)]
#[aliases(QueryGame = QueryPage<QueryFindGame>)]
#[aliases(QueryCollection=QueryPage<QueryFindCollections>)]
pub struct QueryPage<T> {
	pub search: String,
	pub page: u64,
	pub size: u64,
	pub order_by: String,
	pub desc: bool,
	pub query: T,
}
