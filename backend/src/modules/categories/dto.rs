use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

use shared::categories::Categories;
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
#[allow(non_snake_case)]
pub struct CategoriesDTO {
	#[schema(format = "object-id",value_type=String )]
	pub id: Option<ObjectId>,
	pub name: String,
	pub slug: String,
	#[schema(format = "date-time",value_type=String )]
	pub create_at: i64,
}
#[allow(non_snake_case)]
impl From<Categories> for CategoriesDTO {
	fn from(value: Categories) -> Self {
		CategoriesDTO {
			id: value.id,
			name: value.name,
			slug: value.slug,
			create_at: value.created_at.timestamp_millis(),
		}
	}
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct QueryCategory {
	pub name: String,
}
