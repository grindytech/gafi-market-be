use crate::{
    common::ErrorResponse,
    models::{self, game::Game},
};

use super::dto::GameDTO;
use actix_web::Result;
use log::info;
use mongodb::{bson::doc, Collection, Database};
pub async fn get_game_by_id(
    game_id: &String,
    db: Database,
) -> Result<Option<Game>, mongodb::error::Error> {
    let col: Collection<Game> = db.collection(models::game::NAME);
    let filter = doc! {"game_id":game_id};
    col.find_one(filter, None).await
}

pub async fn find_game_account(address: &String, db: Database)
/*  -> Result<Option<Game>, mongodb::error::Error> */
{
    let filter = doc! {"address":address};
    let col: Collection<Game> = db.collection(models::game::NAME);
    let mut curror = col.find(filter, None).await;
    print!("{:?}", curror);
}
