use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, ToSchema, IntoParams)]
pub struct GetNonce {
	/// Address  account
	#[param(example=json!("5DSGohJ8jyaL51woCG6NmSgafnGNWsevHzttJUHdroR8Uh2k"))]
	pub address: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, ToSchema, IntoParams)]
#[into_params(parameter_in=Query)]
pub struct QueryAuth {
	pub address: String,
	pub signature: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
pub struct QueryNonce {
	pub username: String,
	pub login_message: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
pub struct TokenDTO {
	pub access_token: String,
	pub refresh_token: String,
}
