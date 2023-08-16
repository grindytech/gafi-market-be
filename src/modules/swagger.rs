use crate::modules::account::dto::{AccountDTO, SocialInfoDto};
use crate::modules::collection::dto::NFTCollectionDTO;
use crate::modules::game::dto::{GameDTO};
use crate::modules::nft::dto::NFTDTO;
use utoipa::OpenApi;


#[derive(OpenApi)]
#[openapi(
    paths(
      crate::modules::account::controller::get_account,    
     crate::modules::game::controller::get_games_by_address,
      crate::modules::game::controller::get_game,
      crate::modules::nft::controller::get_nft,
      crate::modules::collection::controller::get_collection,
    ),
    components(
        schemas(
            AccountDTO,
            GameDTO,
            SocialInfoDto,
            NFTDTO,
            NFTCollectionDTO
          
           
        )
    ),
    servers(
        (url = "/api/v1"),
    ),
)]

pub struct ApiDoc;
