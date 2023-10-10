use utoipa::{
	openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
	Modify,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        health_checker_handler
    ),
    components(
        schemas(Response)
    ),
    tags(
        (name = "Rust REST API", description = "Authentication in Rust Endpoints")
    ),
    modifiers(&SecurityAddon)
)]
struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
	fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
		let components = openapi.components.as_mut().unwrap();
		components.add_security_scheme(
			"token",
			SecurityScheme::Http(
				HttpBuilder::new().scheme(HttpAuthScheme::Bearer).bearer_format("JWT").build(),
			),
		)
	}
}
