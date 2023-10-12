use slug::slugify;

#[tokio::test]
async fn test_create_category() {
	let (mut db_process, db) = shared::tests::utils::get_test_db(60000).await;
	let category_test = "Gara";

	let slug = slugify(category_test);
	let create = super::service::create_category(slug, db).await;

	println!("Create {:?}", create);

	let _ = db_process.kill();
}

#[tokio::test]
async fn test_get_list_category() {
	let (mut db_process, db) = shared::tests::utils::get_test_db(60000).await;
	let result = super::service::get_categories(db).await;
	print!("List Category {:?}", result);
	let _ = db_process.kill();
}
