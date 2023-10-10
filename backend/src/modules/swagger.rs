use crate::{
	common::{NoResponse, Page, QueryPage, ResponseBody},
	modules::{
		account::dto::AccountDTO,
		collection::dto::NFTCollectionDTO,
		game::dto::GameDTO,
		nft::dto::NFTDTO,
        nft::dto::NFTOwnerOfDto,
	},
};


use shared::{SocialInfo,Favorites, Property, LootTable, LootTableNft};
use utoipa::OpenApi;

use super::{
	auth::dto::{QueryAuth, QueryNonce, TokenDTO},
	game::dto::QueryFindGame,
	pool::dto::{PoolDTO, QueryFindPool},
	transaction::dto::QueryFindTX, account::dto::QueryFindAccount, collection::dto::QueryFindCollections, nft::dto::QueryFindNFts,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::modules::account::controller::get_account,
        crate::modules::account::controller::update_favorite,
        crate::modules::game::controller::search_games_by_query,
        crate::modules::categories::controller::create_new_category,
        crate::modules::categories::controller::get_list_categories,
        crate::modules::game::controller::get_game,
        crate::modules::nft::controller::get_nft,
        crate::modules::nft::controller::get_list_nft,
        crate::modules::nft::controller::search_list_nfts,
        crate::modules::collection::controller::get_collection,
        crate::modules::collection::controller::search_list_collections,
        crate::modules::transaction::controller::get_history_tx,
        crate::modules::transaction::controller::search_history_tx,
        crate::modules::auth::controller::get_random_nonce,
        crate::modules::auth::controller::get_verify_token,
        crate::modules::auth::controller::refresh_token,
        crate::modules::auth::controller::logout,
        crate::modules::pool::controller::search_list_pools,
    ),
   /*  tags(
            (name = "CollectionEndpoints", description = "NFT Collections  endpoints.")
    ), */
    components(
        schemas(
            Favorites,
            AccountDTO,
            GameDTO,
            PoolDTO,
            SocialInfo,
            Property,
            LootTable,
            LootTableNft,
            NFTDTO,
            NFTCollectionDTO,
            ResponseBody<()>,
            Page<()>,
            QueryNonce,
            QueryAuth,
            QueryFindAccount,
            QueryFindGame,
            QueryFindTX,
            QueryFindNFts,
            QueryFindPool,
            QueryFindCollections,
            QueryPage<()>,
            NoResponse,
            NFTOwnerOfDto,
            TokenDTO,
        
        )
    ),
    servers(
        (url = "/api/v1"),
    ),
)]

pub struct ApiDoc;
