use crate::{
	app_state::AppState,
	common::{
		utils::{generate_jwt_token, generate_random_six_digit_number},
		ResponseBody,
	},
	modules::auth::{
		dto::{QueryAuth, QueryNonce},
		service::update_nonce,
	},
};
use actix_web::{
	get,
	http::StatusCode,
	post,
	web::{self, Data},
	Error as AWError, HttpResponse, Result,
};

use super::service::get_jwt_token;

#[utoipa::path(
        tag = "AuthenticationEndpoints",
        context_path = "/auth",
        params((
			"address"=String,Path,description="ID of account",example="0sxbdfc529688922fb5036d9439a7cd61d61114f600"
		)),
        responses(
            (status = OK, description = "Nonce Return Data", body =  ResponseBody<QueryAuth>),
            (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Error",body=NoData)
        ),
)]
#[get("/nonce/{address}")]
pub async fn get_random_nonce(
	app_state: Data<AppState>,
	path: web::Path<String>,
) -> Result<HttpResponse, AWError> {
	let nonce = generate_random_six_digit_number();
	let address = path.into_inner();

	let result = update_nonce(&address, nonce, app_state.db.clone()).await;
	let data = QueryNonce { address, nonce };
	let rsp = ResponseBody::<QueryNonce>::new("", data, true);
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
            "signature":924545
        })
    ),
    responses(
        (status=StatusCode::OK,description="Authentication Success",body=ResponseBody),
        (status=401,description="Authentication Failed",body=NoData)
    )
)]
#[post("/token")]
pub async fn get_verify_token(
	app_state: Data<AppState>,
	req: web::Json<QueryAuth>,
) -> Result<HttpResponse, AWError> {
	let result = get_jwt_token(req.0.clone(), app_state.db.clone()).await;

	match result {
		Ok(Some(account)) => {
			let access_token =
				generate_jwt_token(req.0.clone().address, req.0.signature.to_string());

			let rsp = ResponseBody::<String>::new("Authorizied", access_token.unwrap(), true);

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
