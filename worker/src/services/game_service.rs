use mongodb::{
	bson::{doc, Bson, DateTime},
	options::UpdateOptions,
	results::UpdateResult,
	Database,
};
use serde_json::Value;
use shared::{utils::serde_json_to_doc, BaseDocument, Game, NFTCollection};

pub async fn add_collection(
	game_id: &str,
	collection_id: &str,
	db: &Database,
) -> shared::Result<()> {
	let game_db = db.collection::<Game>(Game::name().as_str());
	let collection_db = db.collection::<NFTCollection>(NFTCollection::name().as_str());
	let game = game_db
		.find_one(
			doc! {
				"game_id": game_id.to_string(),
			},
			None,
		)
		.await?
		.expect("Game not found");
	let collection = collection_db
		.find_one(
			doc! {
				"collection_id": collection_id.to_string(),
			},
			None,
		)
		.await?
		.expect("NFTCollection not found");
	let mut collections: Vec<String> = match game.collections {
		Some(c) => c,
		None => vec![],
	};
	let mut games: Vec<String> = match collection.games {
		Some(g) => g,
		None => vec![],
	};
	collections.push(collection_id.to_string());
	games.push(game_id.to_string());
	game_db
		.update_one(
			doc! {
				"game_id": game_id.to_string(),
			},
			doc! {
				"$set":{"collections": collections,}
			},
			None,
		)
		.await?;
	collection_db
		.update_one(
			doc! {
				"collection_id": collection_id.to_string(),
			},
			doc! {
				"$set": {"games": games,}
			},
			None,
		)
		.await?;
	Ok(())
}

pub async fn remove_collection(
	game_id: &str,
	collection_id: &str,
	db: &Database,
) -> shared::Result<()> {
	let game_db = db.collection::<Game>(Game::name().as_str());
	let collection_db = db.collection::<NFTCollection>(NFTCollection::name().as_str());
	let game = game_db
		.find_one(
			doc! {
				"game_id": game_id.to_string(),
			},
			None,
		)
		.await?
		.expect("Game not found");
	let collection = collection_db
		.find_one(
			doc! {
				"collection_id": collection_id.to_string(),
			},
			None,
		)
		.await?
		.expect("NFTCollection not found");
	let collections: Vec<String> = match game.collections {
		Some(c) => c.into_iter().filter(|c| *c != collection_id.to_string()).collect(),
		None => vec![],
	};
	let games: Vec<String> = match collection.games {
		Some(g) => g.into_iter().filter(|g| *g != game_id.to_string()).collect(),
		None => vec![],
	};
	game_db
		.update_one(
			doc! {
				"game_id": game_id.to_string(),
			},
			doc! {
				"$set":{"collections": collections,}
			},
			None,
		)
		.await?;
	collection_db
		.update_one(
			doc! {
				"collection_id": collection_id.to_string(),
			},
			doc! {
				"$set":{"games": games,}
			},
			None,
		)
		.await?;
	Ok(())
}

pub async fn upsert_game_without_metadata(
	game_id: &str,
	who: &str,
	db: &Database,
) -> shared::Result<UpdateResult> {
	let game_db = db.collection::<Game>(Game::name().as_str());
	let option = UpdateOptions::builder().upsert(true).build();
	let query = doc! {"game_id": game_id.to_string()};
	let new_game = doc! {
		"$set": {
		"game_id": game_id.to_string(),
		"owner": who,
		"updated_at": Some(DateTime::now()),
	  },
	};
	let rs = game_db.update_one(query, new_game, Some(option)).await?;
	log::info!("Game created {} {}", game_id, who);
	Ok(rs)
}

pub async fn get_game_by_id(
	id: &str,
	db: &Database,
) -> Result<Option<Game>, mongodb::error::Error> {
	let game_db = db.collection::<Game>(Game::name().as_str());
	let game = game_db
		.find_one(
			doc! {
			  "game_id": id,
			},
			None,
		)
		.await?;
	Ok(game)
}
pub async fn update_metadata(
	metadata: String,
	game: u32,
	db: &Database,
) -> Result<UpdateResult, mongodb::error::Error> {
	let parsed: Result<Value, serde_json::Error> = serde_json::from_str(&metadata);
	let update;
	match parsed {
		Ok(data) => {
			let parsed_obj = serde_json_to_doc(data);
			match parsed_obj {
				Ok((_doc, obj)) => {
					let empty_val = Value::String("".to_string());
					let banner = obj.get("banner").unwrap_or(&empty_val).as_str().unwrap_or("");
					let logo = obj.get("logo").unwrap_or(&empty_val).as_str().unwrap_or("");
					let cover = obj.get("cover").unwrap_or(&empty_val).as_str().unwrap_or("");
					let description =
						obj.get("description").unwrap_or(&empty_val).as_str().unwrap_or("");
					let name = obj.get("name").unwrap_or(&empty_val).as_str().unwrap_or("");
					let category = obj.get("category").unwrap_or(&empty_val).as_str().unwrap_or("");
					update = doc! {
							"$set": {
							"banner": banner.to_string(),
							"logo": logo.to_string(),
							"cover":cover.to_string(),
							"description": description.to_string(),
							"name": name.to_string(),
							"category":category.to_string(),
							"updated_at": DateTime::now(),
						}
					};
				},
				Err(_) => {
					update = doc! {
							"$set": {
							"banner": Bson::Null,
							"logo": Bson::Null,
							"cover":Bson::Null,
							"category":Bson::Null,
							"description": Bson::Null,
							"name":  Bson::Null,
							"updated_at": DateTime::now(),
						}
					};
				},
			}
		},
		Err(_) => {
			update = doc! {"$set": {
				"updated_at": DateTime::now(),
				"banner_url": Bson::Null,
				"logo_url": Bson::Null,
				"description": Bson::Null,
				"name":  Bson::Null,
			}};
		},
	}
	let game_db: mongodb::Collection<Game> = db.collection::<Game>(Game::name().as_str());
	let option = UpdateOptions::builder().upsert(true).build();
	let query = doc! {"game_id": game.to_string()};
	let rs = game_db.update_one(query, update, option).await?;
	Ok(rs)
}

pub async fn clear_metadata(
	game_id: &str,
	db: &Database,
) -> Result<UpdateResult, mongodb::error::Error> {
	let game_db: mongodb::Collection<Game> = db.collection::<Game>(Game::name().as_str());
	let query = doc! {"game_id": game_id.to_string()};
	let update = doc! {
			"$set": {
				"updated_at": DateTime::now(),
				"banner": Bson::Null,
				"logo": Bson::Null,
				"description": Bson::Null,
				"name":  Bson::Null,
		}
	};
	let rs = game_db.update_one(query, update, None).await?;
	Ok(rs)
}
