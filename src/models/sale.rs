use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Sale {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub sale_id: String,
    pub token_id: ObjectId,
    pub quantity: i16,
    pub creator: String,
    pub type_sale: String,
    pub method: String,
    pub list_price: i32,
    pub begin_at: i32,
    pub end_at: i32,
    pub update_at: i32,
    pub create_at: i32,
}
pub const NAME: &str = "sale";
