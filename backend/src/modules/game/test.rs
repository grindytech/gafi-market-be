#[tokio::test]
async fn test_find_collections() {
	let (mut db_process, db) = shared::tests::utils::get_test_db(60000).await;
    
	let _ = db_process.kill();
}
