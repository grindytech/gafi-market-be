use std::future::{ready, Ready};

use actix_web::{
	dev::Payload,
	error::ErrorUnauthorized,
	http,
	web::{self},
	Error as ActixWebError, FromRequest, HttpRequest,
};

use crate::{
	app_state::AppState,
	common::{utils::verify_jwt_token, ErrorResponse},
};
pub struct JWTMiddleWare {
	pub address: String,
}
impl FromRequest for JWTMiddleWare {
	type Error = ActixWebError;
	type Future = Ready<Result<Self, Self::Error>>;

	fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
		let data = req.app_data::<web::Data<AppState>>().unwrap();

		//seperate get token from cookie
		let token = req.cookie("token").map(|c| c.value().to_string()).or_else(|| {
			req.headers()
				.get(http::header::AUTHORIZATION)
				.map(|h| h.to_str().unwrap().split_at(7).1.to_string())
		});

		/* 		log::info!("Token {:?}", token); */
		if token.is_none() {
			let json_error = ErrorResponse {
				status: "fail".to_string(),
				message: "Fail Verify Valid Token, please login".to_string(),
			};
			return ready(Err(ErrorUnauthorized(json_error)));
		}
		let claims = verify_jwt_token(token.unwrap(), data.config.clone());

		match claims {
			Ok(value_token_payload) => {
				let address = value_token_payload.address;
				ready(Ok(JWTMiddleWare { address }))
			},
			Err(e) => {
				log::info!("Error {:?}", e);
				let json_error = ErrorResponse {
					status: "fail".to_string(),
					message: "Invalid token".to_string(),
				};
				return ready(Err(ErrorUnauthorized(json_error)));
			},
		}
	}

	fn extract(req: &actix_web::HttpRequest) -> Self::Future {
		Self::from_request(req, &mut actix_web::dev::Payload::None)
	}
}
