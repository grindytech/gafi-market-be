use crate::{
    common::ErrorResponse,
    models::{self, game::Game},
};

use super::dto::GameDTO;
use actix_web::Result;
use futures_util::TryStreamExt;
/* use futures::stream::StreamExt; */
use log::info;
use mongodb::{
    bson::{self, doc, Bson, Document},
    options, Collection, Cursor, Database,
};
pub async fn get_game_by_id(
    game_id: &String,
    db: Database,
) -> Result<Option<Game>, mongodb::error::Error> {
    let col: Collection<Game> = db.collection(models::game::NAME);
    let filter = doc! {"game_id":game_id};
    col.find_one(filter, None).await
}

pub async fn find_games_account(
    address: &String,
    db: Database,
) -> Result<Option<Vec<GameDTO>>, mongodb::error::Error> {
    let filter = doc! {"owner":address};
    let col: Collection<Game> = db.collection(models::game::NAME);
    /*   let option = options::FindOptions::default(); */
    let mut cursor = col.find(filter, None).await?;
    let mut list_games: Vec<GameDTO> = Vec::new();

    while let Some(game) = cursor.try_next().await? {
        list_games.push(game.into())
    }

    Ok(Some(list_games))
}
