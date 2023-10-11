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
	common::{
		utils::{verify_access_token, verify_refresh_token},
		ErrorResponse,
	},
};
pub struct JWTMiddleWare {
	pub address: String,
	pub token: String,
}
impl JWTMiddleWare {
	fn extract_token(req: &HttpRequest, name_cookie: &str) -> Option<String> {
		// Try to extract the access token from a cookie
		let cookie_token = req.cookie(name_cookie).map(|c| c.value().to_string());

		// Try to extract the access token from the "Authorization" header
		let header_token = req.headers().get(http::header::AUTHORIZATION).and_then(|header| {
			if let Ok(auth_str) = header.to_str() {
				if auth_str.starts_with("Bearer ") {
					Some(auth_str[7..].to_string())
				} else {
					None
				}
			} else {
				None
			}
		});

		// Return the first successfully extracted token
		cookie_token.or(header_token)
	}
}

impl FromRequest for JWTMiddleWare {
	type Error = ActixWebError;
	type Future = Ready<Result<Self, Self::Error>>;

	fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
		let data = req.app_data::<web::Data<AppState>>().unwrap();

		let access_token = JWTMiddleWare::extract_token(req, "access_token");

		if access_token.is_none() {
			let json_error = ErrorResponse {
				status: "fail".to_string(),
				message: "Fail Verify Valid Token, please login".to_string(),
			};
			return ready(Err(ErrorUnauthorized(json_error)));
		}
		let claims = verify_access_token(access_token.clone().unwrap(), data.config.clone());

		match claims {
			Ok(value_token_payload) => {
				let address = value_token_payload.address;

				ready(Ok(JWTMiddleWare {
					address,
					token: access_token.unwrap(),
				}))
			},
			Err(e) => match *e.kind() {
				jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
					let refresh_token = JWTMiddleWare::extract_token(req, "refresh_token");
					let refresh_result =
						verify_refresh_token(refresh_token.unwrap(), data.config.clone());

					// Attempt to refresh the access token
					match refresh_result {
						Ok(new_access_token) => {
							let address = new_access_token.address;

							ready(Ok(JWTMiddleWare {
								address,
								token: access_token.unwrap(),
							}))
						},
						Err(_) => {
							let json_error = ErrorResponse {
								status: "fail".to_string(),
								message: "Invalid token".to_string(),
							};
							return ready(Err(ErrorUnauthorized(json_error)));
						},
					}
					/* let json_error = ErrorResponse {
						status: "fail".to_string(),
						message: "Access Token has expired".to_string(),
					};
					return ready(Err(actix_web::error::ErrorUnauthorized(json_error))); */
				},
				_ => {
					let json_error = ErrorResponse {
						status: "fail".to_string(),
						message: "Invalid token".to_string(),
					};
					return ready(Err(ErrorUnauthorized(json_error)));
				},
			},
		}
	}

	fn extract(req: &actix_web::HttpRequest) -> Self::Future {
		Self::from_request(req, &mut actix_web::dev::Payload::None)
	}
}
// !todo Refactor from request check
