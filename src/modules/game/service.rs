use crate::models::{self, game::Game};

use actix_web::Result;
use mongodb::{bson::doc, Collection, Database};

use super::dto::GameDTO;
pub async fn get_game(
    game_id: &String,
    db: Database,
) -> Result<Option<Game>, mongodb::error::Error> {
    let col: Collection<Game> = db.collection(models::game::NAME);
    let filter = doc! {"game_id":game_id};
    col.find_one(filter, None).await
}
