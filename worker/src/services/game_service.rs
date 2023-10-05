use mongodb::{
	bson::{doc, Bson, DateTime},
	options::UpdateOptions,
	results::UpdateResult,
	Database,
};
use serde_json::Value;
use shared::{
	utils::serde_json_to_properties,
	BaseDocument, Game, NFTCollection,
};

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
			let parsed_obj = serde_json_to_properties(data);
			match parsed_obj {
				Ok((doc, _properties, _obj)) => {
					update = doc! {
							"$set": {
							"updated_at": DateTime::now(),
							"metadata": metadata.clone(),
							"attributes": doc,
						}
					};
				},
				Err(_) => {
					update = doc! {
							"$set": {
							"updated_at": DateTime::now(),
							"metadata": metadata.clone(),
							"attributes": Bson::Null,
						}
					};
				},
			}
		},
		Err(_) => {
			update = doc! {"$set": {
				"updated_at": DateTime::now(),
				"metadata": metadata.clone(),
				"attributes": Bson::Null,
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
				"metadata": Bson::Null,
				"attributes": Bson::Null,
		}
	};
	let rs = game_db.update_one(query, update, None).await?;
	Ok(rs)
}
