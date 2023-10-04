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
	pub createdAt: DateTime,
}
#[allow(non_snake_case)]
impl From<Categories> for CategoriesDTO {
	fn from(value: Categories) -> Self {
		CategoriesDTO {
			id: value.id,
			name: value.name,
			slug: value.slug,
			createdAt: value.created_at,
		}
	}
}
impl Into<Categories> for CategoriesDTO {
	fn into(self) -> Categories {
		Categories {
			id: self.id,
			name: self.name,
			slug: self.slug,
			created_at: self.createdAt,
		}
	}
}
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct QueryCategory {
	pub name: String,
}
