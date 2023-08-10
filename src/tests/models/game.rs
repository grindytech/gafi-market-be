use crate::{
    models::{
        self,
        account::{self, Account, SocialInfo},
        game::Game,
    },
    utils::{config::Config, db},
};
use actix_web::test;
use dotenv::dotenv;
use mongodb::{
    bson::{doc, oid::ObjectId},
    Collection,
};
use mongodb::{Client, IndexModel};
#[actix_web::test]
async fn test() {
    dotenv().ok();
    let configuration = Config::init();
    let database = db::get_database(
        configuration.mongodb_uri.clone(),
        configuration.mongodb_db_name.clone(),
    )
    .await;
    // init collection
    let col: Collection<Game> = database.collection(models::game::NAME);
    let social_objects = SocialInfo {
        twitter: Some("karas".to_string()),
        web: Some("https://example.com".to_string()),
        medium: None,
        facebook: None,
        discord: None,
    };
    let game_test=Game{
        id:Some(ObjectId::new()),
        game_id:"123".to_string(),
        owner:"0sxbdfc529688922fb5036d9439a7cd61d61114f300".to_string(),
        is_verified:true,
        name:"HYTOPIA Worlds 300".to_string(),
        slug:"topia-worlds-300".to_string(),
        social:social_objects,
        description:"HYTOPIA is a game and creator platform developed by Minecraft modding experts, aiming to overcome Minecraft's limitations and become the next, Minecraft. The platform promotes innovation a snd collaboration among players, creators, and contributors, fostering an interconnected ecosystem with a new game engine and resources. HYTOPIA's mission is to create the largest UGC (user-generated content) games platform in the world.HYTOPIA is comprised of 10,000 worlds - a world is required to create, launch and monetize a massively multiplayer game within the HYTOPIA ecosystem.".to_string(),
        category:"web 3".to_string(),
        banner_url:Some("https://i.seadn.io/gcs/files/9320ea72fccb2dd63b119fc66f941580.png?auto=format&dpr=1&w=3840".to_string()),
        logo_url:Some("https://i.seadn.io/gcs/files/b14329da267669950c65d95b030a305f.png?auto=format&dpr=1&w=384".to_string()),
        update_at:1234867890,
        create_at:1234567890,
    };
    col.insert_one(game_test, None)
        .await
        .expect("Create New Game");
}
