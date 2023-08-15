use actix_web::web::Data;
use mongodb::{bson::doc, Collection, Database};

use crate::{
    app_state::AppState,
    models::{self, game::Game},
};

use log::{info, warn};

pub async fn find_game_of_account(address: &String, db: Database)
/*  -> Result<Option<Game>, mongodb::error::Error> */
{
    let filter = doc! {"address":address};
    let col: Collection<Game> = db.collection(models::game::NAME);
    log::info!("???????");
    let mut curror = col.find(filter, None).await;
    print!("{:?}", curror);
}
#[actix_web::test]
async fn test() {
    let db = crate::tests::utils::get_database().await;
    let address = "0sxbdfc529688922fb5036d9439a7cd61d61114f700".to_string();
    find_game_of_account(&address, db);
}
