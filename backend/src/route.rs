use crate::modules;
use actix_web::web::{self, scope};
use utoipa::OpenApi;
use utoipa_swagger_ui::{SwaggerUi, Url};
pub fn route_config(cfg: &mut web::ServiceConfig) {
	cfg.service(
		web::scope("/api/v1")
			.service(modules::nft::controller::endpoints(scope("/nft")))
			.service(modules::game::controller::endpoints(scope("/game")))
			.service(modules::account::controller::endpoints(scope("/account")))
			.service(modules::collection::controller::endpoints(scope(
				"/collection",
			)))
			.service(modules::bundle::controller::endpoints(scope("/bundle")))
			.service(modules::transaction::controller::endpoints(scope("/tx"))),
	)
	.service(SwaggerUi::new("/swagger-ui/{_:.*}").urls(vec![(
		Url::new("v1", "/api-docs/api.json"),
		modules::swagger::ApiDoc::openapi(),
	)]));
}
