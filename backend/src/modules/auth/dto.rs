use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
pub struct QueryAuth {
	pub address: String,
	pub signature: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
pub struct QueryNonce {
	pub address: String,
	pub nonce: u32,
}