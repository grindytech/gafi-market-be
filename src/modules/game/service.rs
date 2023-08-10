use crate::models::{self, game::Game};

use actix_web::Result;
use mongodb::{
    bson::{doc, extjson::de::Error},
    Collection, Database,
};

use super::dto::{GameDTO, SocialDTO};
pub async fn get_game(game_id: &String, db: Database) -> Result<Option<GameDTO>, Error> {
    let col: Collection<Game> = db.collection(models::game::NAME);
    let filter = doc! {"game_id":game_id};
    if let Ok(Some(game_detail)) = col.find_one(filter, None).await {
        let game_dto = GameDTO {
            game_id: game_detail.game_id.to_string(),
            name: game_detail.name,
            owner: game_detail.owner,
            social: SocialDTO {
                twitter: game_detail.social.twitter.clone(),
                discord: game_detail.social.discord.clone(),
                facebook: game_detail.social.facebook.clone(),
                medium: game_detail.social.medium.clone(),
                web: game_detail.social.web.clone(),
            },
            category: game_detail.category,
            description: game_detail.description,
            logo_url: game_detail.logo_url,
            banner_url: game_detail.banner_url,
            create_at: game_detail.create_at,
            is_verified: false,
        };
        Ok(Some(game_dto))
    } else {
        Ok(None)
    }
}
