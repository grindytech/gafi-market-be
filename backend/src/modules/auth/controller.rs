use crate::{
	app_state::{self, AppState},
	common::{
		utils::{
			generate_jwt_token, generate_message_sign_in, generate_uuid, 
		},
		ResponseBody,
	},
	middleware,
	modules::auth::{
		dto::QueryAuth,
		dto::{GetNonce, QueryNonce, TokenDTO},
		service::{delete_refresh_token, update_nonce, verify_signature},
	},
};
use actix_web::{
	get,
	http::StatusCode,
	post,
	web::{self, Data},
	Error as AWError, HttpResponse, Result,
};

#[utoipa::path(
	get,
        tag = "AuthenticationEndpoints",
        context_path = "/auth",
        params(
		GetNonce
		),
        responses(
            (status = OK, description = "Authentication Message", body =  QueryNonce),
            (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Error",body=NoData)
        ),
)]
#[get("/nonce")]
pub async fn get_random_nonce(
	app_state: Data<AppState>,
	query: web::Query<GetNonce>,
) -> Result<HttpResponse, AWError> {
	if query.address.len() < 32 {
		return Ok(
			HttpResponse::BadRequest().json("Address must have a minimum length of 32 characters.")
		);
	}

	let nonce = generate_uuid();
	let address = query.0.address;
	let result = update_nonce(&address, nonce.clone(), app_state.db.clone()).await;

	let data = generate_message_sign_in(&address, &nonce);

	let rsp = ResponseBody::<QueryNonce>::new(
		"Signature Request",
		QueryNonce {
			login_message: data,
			username: address,
		},
		true,
	);
	Ok(HttpResponse::build(StatusCode::OK).content_type("application/json").json(rsp))
}

#[utoipa::path(
    post,
    tag="AuthenticationEndpoints",
    context_path="/auth",
    request_body(
        content=QueryAuth,
        description="Verify Token",
        example=json!({
            "address":"0sxbdfc529688922fb5036d9439a7cd61d61114f600",
            "signature":"",

        })
    ),
    responses(
        (status=StatusCode::OK,description="Authentication Success",body=TokenData),
        (status=401,description="Authentication Failed",body=NoData)
    )
)]
#[post("/token")]
pub async fn get_verify_token(
	app_state: Data<AppState>,
	req: web::Json<QueryAuth>,
) -> Result<HttpResponse, AWError> {
	let result = verify_signature(req.0.clone(), app_state.clone()).await;

	match result {
		Ok(Some(account)) => {
			let access_token = generate_jwt_token(
				req.0.clone().address,
				app_state.config.clone(),
				app_state.config.jwt_access_time,
			);
			log::info!("Access Token {:?} ", access_token);
			let rsp = ResponseBody::<TokenDTO>::new(
				"Authorizied",
				TokenDTO {
					access_token: access_token.unwrap_or("access token error".to_string()),
					refresh_token: account.refresh_token.unwrap(),
				},
				true,
			);

			Ok(HttpResponse::build(StatusCode::OK).content_type("application/json").json(rsp))
		},
		Ok(None) => {
			let rsp: ResponseBody<Option<_>> =
				ResponseBody::<Option<()>>::new("Unauthenticated", None, false);
			Ok(HttpResponse::build(StatusCode::UNAUTHORIZED)
				.content_type("application/json")
				.json(rsp))
		},
		Err(e) => {
			let rsp = ResponseBody::<Option<()>>::new(e.to_string().as_str(), None, false);
			Ok(HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(rsp))
		},
	}
}

#[utoipa::path(
	post,
	tag="AuthenticationEndpoints",
	context_path="/auth",

	responses(
		(status=StatusCode::OK,description="Reresh Token Success",body=TokenData),
		(status=401,description="Refresh token Failed",body=NoData)
	)
)]
#[post("/refresh_token")]
pub async fn refresh_token(
	app_state: Data<AppState>,
	auth: middleware::JWTMiddleWare,

) -> Result<HttpResponse, AWError> {
	let access_token = generate_jwt_token(
				auth.address,
				app_state.config.clone(),
				app_state.config.jwt_access_time,
			);
	match access_token {
		Ok(value) => {
			
			Ok(HttpResponse::build(StatusCode::OK).content_type("application/json").json(value))
		},
		Err(e) => {
			let rsp = ResponseBody::<Option<()>>::new(e.to_string().as_str(), None, false);
			Ok(HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(rsp))
		},
	}
}

#[utoipa::path(
	post,
	tag="AuthenticationEndpoints",
	context_path="/auth",
	
	responses(
		(status=StatusCode::OK,description="LogOut Success",body=TokenData),
		(status=401,description="Logout Failed",body=NoData)
	)
)]
#[post("/logout")]
pub async fn logout(
	app_state: Data<AppState>,
	auth: middleware::JWTMiddleWare,
) -> Result<HttpResponse, AWError> {
	let result = delete_refresh_token(auth.address.to_string(), app_state.clone()).await;
	match result {
		Ok(Some(value)) => {
			let rsp: ResponseBody<Option<_>> =
				ResponseBody::<Option<()>>::new("Logout Success", None, true);
			Ok(HttpResponse::build(StatusCode::OK).content_type("application/json").json(rsp))
		},
		Ok(None) => {
			let rsp: ResponseBody<Option<_>> =
				ResponseBody::<Option<()>>::new("Logout Failed", None, true);
			Ok(HttpResponse::build(StatusCode::BAD_REQUEST)
				.content_type("application/json")
				.json(rsp))
		},
		Err(e) => {
			let rsp = ResponseBody::<Option<()>>::new(e.to_string().as_str(), None, false);
			Ok(HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(rsp))
		},
	}
}

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
	scope
		.service(get_random_nonce)
		.service(get_verify_token)
		.service(refresh_token)
		.service(logout)
}
