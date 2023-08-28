use crate::{
	app_state::AppState,
	common::{ResponseBody, QueryPage},
	modules::account::{dto::{AccountDTO, QueryFindAccount}, service::{find_account_by_adress, update_favorites_account}},
	shared::constant::EMPTY_STR,
};
use actix_web::{
	get,
	http::StatusCode,
	web::{self, Data},
	Error as AWError, HttpResponse, Result, post,
};

#[utoipa::path(
        tag = "AccountEndpoints",
        context_path = "/account",
        params((
			"data_id"=String,Path,description="ID of account",example="0sxbdfc529688922fb5036d9439a7cd61d61114f600"
		)),
        responses(
            (status = OK, description = "Account Response", body = AccountObject)
        ),
    )]
#[get("/{data_id}")]
pub async fn get_account(
	app_state: Data<AppState>,
	path: web::Path<String>,
) -> Result<HttpResponse, AWError> {
	let data_id = path.into_inner();
	let account_detail = find_account_by_adress(&data_id, app_state.db.clone()).await;
	match account_detail {
		Ok(Some(account)) => {
			let rsp = ResponseBody::<Option<AccountDTO>>::new(EMPTY_STR, Some(account), true);
			Ok(HttpResponse::build(StatusCode::OK).content_type("application/json").json(rsp))
		},
		Ok(None) => {
			let rsp = ResponseBody::<Option<AccountDTO>>::new("Not found", None, false);
			Ok(HttpResponse::build(StatusCode::NOT_FOUND)
				.content_type("application/json")
				.json(rsp))
		},
		Err(e) => {
			let rsp = ResponseBody::<Option<AccountDTO>>::new(e.to_string().as_str(), None, false);
			Ok(HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(rsp))
		},
	}
}


#[utoipa::path(
	post,
	tag="AccountEndpoints",
	context_path="/account",
	request_body(
		content=QueryFindAccount,description="Update New Favorite",
		example=json!({
			"search":"",
			"page": 1,
        	"size": 10,
        	"order_by": "create_at",
        	"desc": true,
			"query":{
				"name":null,
				"address":"5DhYYp1Q2sNXR7HfzbQFUt3XHfK4CKYRA4vaaKRiWpSLkp62",
				"favorites":[
					{
						"token_id":"0xd774557b647330c91bf44cfeab205095f7e6c367",
						"collection_id":"Q29sbGVjdGlvblR5cGU6MjQxOTc3MTc",
						"amount":40,
					},
					{
						"token_id":"0xd774557b647330c91bf44cfeab205095f7e6c368",
						"collection_id":"Q29sbGVjdGlvblR5cGU6MjQxOTc3MTc",
						"amount":40,
					}
					
				]
			}
		})
	),
	responses(
        (status=StatusCode::OK,description="Update Profilee Success",body=AccountDTO),
        (status=StatusCode::INTERNAL_SERVER_ERROR,description="Error",body=NoData)

   	 )
)]
#[post("/updateFavorite")]
pub async fn update_favorite(
	app_state: Data<AppState>,
	req: web::Json<QueryPage<QueryFindAccount>>,
) -> Result<HttpResponse, AWError> {
	let result = update_favorites_account(req.0, app_state.db.clone()).await;
	match result {
		Ok(Some(account)) => Ok(HttpResponse::build(StatusCode::OK)
			.content_type("application/json")
			.json(account)),
		Ok(None) => {
			let rsp: ResponseBody<Option<_>> =
				ResponseBody::<Option<()>>::new("Not found", None, false);
			Ok(HttpResponse::build(StatusCode::NOT_FOUND)
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
	scope.service(get_account).service(update_favorite)
}
