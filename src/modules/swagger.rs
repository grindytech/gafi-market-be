use crate::modules::account::dto::{AccountDTO, SocialInfoDto};
use crate::modules::game::dto::{GameDTO, SocialDTO};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
      crate::modules::account::controller::get_define_account,
      crate::modules::game::controller::get_define_game
    ),
    components(
        schemas(
            AccountDTO,
            GameDTO,
            SocialDTO,
            SocialInfoDto
        )
    ),
    servers(
        (url = "/api/v1"),
    ),
)]

pub struct ApiDoc;
