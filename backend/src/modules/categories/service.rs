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
	if col.find_one(doc! {"slug": &slug}, None).await?.is_some() {
		return Ok(None);
	};
	let new_category = Categories {
		name,
		slug,
		id: None,
		created_at: DateTime::now(),
	};
	let rs = col.insert_one(&new_category, None).await?;
	Ok(Some(new_category))
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
