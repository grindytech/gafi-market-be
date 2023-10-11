use crate::common::ErrorResponse;

use super::dto::CategoriesDTO;

use futures_util::TryStreamExt;
use mongodb::{
	bson::{doc, DateTime},
	error::ErrorKind,
	Collection, Database,
};

use shared::{categories::Categories, models, BaseDocument};
use slug::slugify;
pub async fn create_category(
	name: String,
	db: Database,
) -> Result<Option<Categories>, mongodb::error::Error> {
	let col: Collection<Categories> =
		db.collection(models::categories::Categories::name().as_str());
	let slug = slugify(&name);
	let exist = col
		.find_one(
			doc! {
				"slug":&slug
			},
			None,
		)
		.await;
	/* match exist {
		Ok(value) => {},
		Err(e) => Err(e),
	}; */
	let new_category = Categories {
		name,
		slug,
		id: None,
		created_at: DateTime::now(),
	};
	let rs = col.insert_one(&new_category, None).await;
	match rs {
		Ok(r) => Ok(Some(new_category)),
		Err(e) => Err(e),
	}
}
pub async fn get_categories(
	db: Database,
) -> Result<Option<Vec<CategoriesDTO>>, mongodb::error::Error> {
	let col: Collection<Categories> =
		db.collection(models::categories::Categories::name().as_str());
	let mut cursor = col.find(doc! {}, None).await?;
	let mut list_categories: Vec<CategoriesDTO> = Vec::new();
	while let Some(category) = cursor.try_next().await? {
		list_categories.push(category.into())
	}
	Ok(Some(list_categories))
}
