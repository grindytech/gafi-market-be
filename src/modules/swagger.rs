use crate::modules::account::dto::{AccountDTO, SocialInfoDto};
use crate::modules::collection::dto::NFTCollectionDTO;
use crate::modules::game::dto::GameDTO;
use crate::modules::nft::dto::NftDTO;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
      crate::modules::account::controller::get_define_account,
      crate::modules::game::controller::get_define_game,
      crate::modules::nft::controller::get_define_nft,
      crate::modules::collection::controller::get_define_collection,
    ),
    components(
        schemas(
            AccountDTO,
            GameDTO,
            SocialInfoDto,
            NftDTO,
            NFTCollectionDTO
        )
    ),
    servers(
        (url = "/api/v1"),
    ),
)]

pub struct ApiDoc;
