use crate::{
	common::{NoResponse, Page, QueryPage, ResponseBody},
	modules::{
		account::dto::{AccountDTO, SocialInfoDto},
		collection::dto::NFTCollectionDTO,
		game::dto::GameDTO,
		nft::dto::NFTDTO,
	},
};
use utoipa::OpenApi;

use super::nft::dto::{PropertiseDTO, QueryFindNFts};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::modules::account::controller::get_account,
        crate::modules::game::controller::get_games_by_address,
        crate::modules::game::controller::get_game,
        crate::modules::nft::controller::get_nft,
        crate::modules::nft::controller::get_list_nft,
        crate::modules::nft::controller::search_list_nfts,
        crate::modules::collection::controller::get_collection,
        
    ),
    components(
        schemas(
            AccountDTO,
            GameDTO,
            SocialInfoDto,
            NFTDTO,
            NFTCollectionDTO,
            PropertiseDTO,
            ResponseBody<()>,
            Page<()>,
            QueryPage<()>,
            QueryFindNFts,
            NoResponse
        )
    ),
    servers(
        (url = "/api/v1"),
    ),
)]

pub struct ApiDoc;
