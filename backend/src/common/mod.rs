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
	pub page: i64,
	pub size: i64,
	pub total: i64,
}

impl<T> Page<T> {
	pub fn new(message: &str, data: Vec<T>, page: i64, size: i64, total: i64) -> Page<T> {
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
pub struct QueryPage<T> {
	pub search: String,
	pub page: i64,
	pub size: i64,
	pub order_by: String,
	pub desc: bool,
	pub query: T,
}
