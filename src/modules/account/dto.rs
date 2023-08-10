use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
pub struct AccountDTO {
    pub address: String,
    pub balance: String,
    pub is_verified: bool,
    pub name: String,
    pub bio: String,
    pub logo_url: Option<String>,
    pub banner_url: Option<String>,
}
