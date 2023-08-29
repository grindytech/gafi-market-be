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

use super::{
	auth::dto::{QueryAuth, QueryNonce},
	game::dto::QueryFindGame,
	nft::dto::PropertiseDTO,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::modules::account::controller::get_account,
        crate::modules::account::controller::update_favorite,
        crate::modules::game::controller::search_games_by_query,
        crate::modules::game::controller::get_game,
        crate::modules::nft::controller::get_nft,
        crate::modules::nft::controller::get_list_nft,
        crate::modules::nft::controller::search_list_nfts,
        crate::modules::collection::controller::get_collection,
        crate::modules::collection::controller::search_list_collections,
        crate::modules::transaction::controller::get_history_tx,
        crate::modules::transaction::controller::search_history_tx,
        crate::modules::auth::controller::get_random_nonce,
        crate::modules::auth::controller::get_verify_token
    ),
   /*  tags(
            (name = "CollectionEndpoints", description = "NFT Collections  endpoints.")
    ), */
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
            QueryNonce,
            QueryAuth,
            QueryFindGame,
        
            QueryPage<()>,
            NoResponse
        )
    ),
    servers(
        (url = "/api/v1"),
    ),
)]

pub struct ApiDoc;
