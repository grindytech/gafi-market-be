use crate::{
	app_state::AppState,
	common::{
		utils::{generate_jwt_token, generate_message_sign_in, generate_uuid},
		ResponseBody,
	},
	modules::auth::{dto::QueryAuth, dto::QueryNonce, service::update_nonce},
};
use actix_web::{
	get,
	http::StatusCode,
	post,
	web::{self, Data},
	Error as AWError, HttpResponse, Result,
};

use super::service::get_access_token;

#[utoipa::path(
        tag = "AuthenticationEndpoints",
        context_path = "/auth",
        params((
			"address"=String,description="ID of account",example="0sxbdfc529688922fb5036d9439a7cd61d61114f600"
		)),
        responses(
            (status = OK, description = "Authentication Message", body =  QueryNonce),
            (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Error",body=NoData)
        ),
)]
#[get("/nonce/{address}")]
pub async fn get_random_nonce(
	app_state: Data<AppState>,
	path: web::Path<String>,
) -> Result<HttpResponse, AWError> {
	let nonce = generate_uuid();
	let address = path.into_inner();

	let result = update_nonce(&address, nonce.clone(), app_state.db.clone()).await;

	let data = generate_message_sign_in(&address, &nonce);

	let rsp = ResponseBody::<QueryNonce>::new(
		"Signature Request",
		QueryNonce {
			login_message: data,
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
        (status=StatusCode::OK,description="Authentication Success",body=String),
        (status=401,description="Authentication Failed",body=NoData)
    )
)]
#[post("/token")]
pub async fn get_verify_token(
	app_state: Data<AppState>,
	req: web::Json<QueryAuth>,
) -> Result<HttpResponse, AWError> {
	let result = get_access_token(req.0.clone(), app_state.clone()).await;

	match result {
		Ok(Some(account)) => {
			let access_token = generate_jwt_token(req.0.clone().address, app_state.config.clone());

			let rsp = ResponseBody::<String>::new("Authorizied", access_token.unwrap(), true); //fix

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
pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
	scope.service(get_random_nonce).service(get_verify_token)
}
