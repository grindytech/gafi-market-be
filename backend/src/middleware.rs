use std::future::{ready, Ready};

use actix_web::{
	dev::Payload,
	error::ErrorUnauthorized,
	http,
	web::{self, Data},
	Error as ActixWebError, FromRequest, HttpMessage,
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

use crate::{
	app_state::AppState,
	common::{ErrorResponse, TokenPayload},
};
pub struct JWTMiddleWare {
	pub address: uuid::Uuid,
}
impl FromRequest for JWTMiddleWare {
	type Error = ActixWebError;
	type Future = Ready<Result<Self, Self::Error>>;

	fn from_request(
		req: &actix_web::HttpRequest,
		payload: &mut actix_web::dev::Payload,
	) -> Self::Future {
		let data = req.app_data::<web::Data<AppState>>().unwrap();

		let token = req.cookie("token").map(|c| c.value().to_string()).or_else(|| {
			req.headers()
				.get(http::header::AUTHORIZATION)
				.map(|h| h.to_str().unwrap().split_at(7).1.to_string())
		});
		if token.is_none() {
			let json_error = ErrorResponse {
				status: "fail".to_string(),
				message: "Fail Verify Valid Token, please login".to_string(),
			};
			return ready(Err(ErrorUnauthorized(json_error)));
		}

		let claims = match decode::<TokenPayload>(
			&token.unwrap(),
			&DecodingKey::from_secret(data.config.jwt_secret_key.as_ref()),
			&&Validation::new(Algorithm::HS256),
		) {
			Ok(c) => c.claims,
			Err(_) => {
				let json_error = ErrorResponse {
					status: "fail".to_string(),
					message: "Invalid token".to_string(),
				};
				return ready(Err(ErrorUnauthorized(json_error)));
			},
		};

		let address = uuid::Uuid::parse_str(claims.address.as_str()).unwrap();
		req.extensions_mut().insert::<uuid::Uuid>(address.to_owned());

		ready(Ok(JWTMiddleWare { address }))
	}

	fn extract(req: &actix_web::HttpRequest) -> Self::Future {
		Self::from_request(req, &mut actix_web::dev::Payload::None)
	}
}
