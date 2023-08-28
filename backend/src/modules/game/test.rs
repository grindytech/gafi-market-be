use futures_util::TryStreamExt;
use mongodb::{bson::doc, Collection, Database};

use log::{info, warn};
use shared::{
	models::{self, game::Game},
	BaseDocument,
};

pub async fn find_games_of_account(
	address: &String,
	db: Database,
) -> Result<Vec<Game>, mongodb::error::Error> {
	let filter = doc! {"address":address};
	let col: Collection<Game> = db.collection(models::Game::name().as_str());
	/* let mut curror = */
	let mut cursor = match col.find(filter, None).await {
		Ok(cursor) => cursor,
		Err(_) => return Ok(vec![]),
	};
	// Iterate over the results of the cursor.
	while let Some(what) = cursor.try_next().await? {
		println!("title: {:?}", what.name);
	}

	Ok(cursor.try_collect().await.unwrap_or_else(|_| vec![]))
}
fn init() {
	let _ = env_logger::builder().is_test(true).try_init();
}
#[actix_web::test]
async fn test() {
	init();
	info!("We");
	let db = shared::tests::utils::get_database().await;
	let address = "0sxbdfc529688922fb5036d9439a7cd61d61114f700".to_string();
	find_games_of_account(&address, db);
}
