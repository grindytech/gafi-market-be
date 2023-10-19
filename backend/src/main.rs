use actix_cors::Cors;
use actix_web::{
	dev,
	http::{header, StatusCode},
	middleware::{ErrorHandlerResponse, ErrorHandlers},
	web::{self},
	App, HttpServer, Result,
};

use app_state::AppState;
use dotenv::dotenv;
use env_logger::Env;

mod app_state;
mod common;
mod middleware;
mod modules;
pub use shared;

pub mod route;

use route::route_config;
use shared::{db, Config};

fn add_error_header<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
	res.response_mut().headers_mut().insert(
		header::CONTENT_TYPE,
		header::HeaderValue::from_static("Error"),
	);
	Ok(ErrorHandlerResponse::Response(res.map_into_left_body()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	dotenv().ok();
	let configuration = Config::init();
	let database = db::get_database(
		configuration.mongodb_uri.clone(),
		configuration.mongodb_db_name.clone(),
	)
	.await;
	db::init_db(database.clone()).await;
	env_logger::init_from_env(Env::default().default_filter_or("info"));

	HttpServer::new(move || {
		let cors = Cors::permissive();
		App::new()
			/* .wrap(Logger::default())
			.wrap(Logger::new("%a %t %r %s %b %T")) */
			.wrap(cors)
			.wrap(ErrorHandlers::new().handler(StatusCode::INTERNAL_SERVER_ERROR, add_error_header))
			.app_data(web::Data::new(AppState {
				db: database.clone(),
				config: configuration.clone(),
			}))
			.configure(route_config)
	})
	.bind(("0.0.0.0", 8080))?
	.run()
	.await
}
