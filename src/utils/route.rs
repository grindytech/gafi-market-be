use crate::modules;
use actix_web::web::{self, scope};
use modules::account;
use modules::game;
use utoipa::OpenApi;
use utoipa_swagger_ui::{SwaggerUi, Url};
pub fn route_config(cfg: &mut web::ServiceConfig) {
    cfg.service(modules::nft::controller::endpoints(scope("/nft")))
        .service(modules::game::controller::endpoints(scope("/game")))
        .service(modules::account::controller::endpoints(scope("/account")))
        .service(SwaggerUi::new("/swagger-ui/{_:.*}").urls(vec![
            (
                Url::new("account", "/api-docs/account.json"),
                account::controller::ApiDoc::openapi(),
            ),
            (
                Url::new("game", "/api-docs/game.json"),
                game::controller::ApiDoc::openapi(),
            ),
        ]));
}
