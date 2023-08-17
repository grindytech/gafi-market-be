use crate::{
    app_state::AppState,
    common::ResponseBody,
    constant::EMPTY_STR,
    modules::account::{dto::AccountDTO, service::get_account_by_adress},
};
use actix_web::{
    get,
    http::StatusCode,
    web::{self, Data},
    Error as AWError, HttpResponse, Result,
};

#[utoipa::path(
        tag = "account",
        context_path = "/account",
        params(("data_id"=String,Path,description="ID of account",example=json!({"query":{"address":"0sxbdfc529688922fb5036d9439a7cd61d61114f600"}}))),
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
    let account_detail = get_account_by_adress(&data_id, app_state.db.clone()).await;
    match account_detail {
        Ok(Some(account)) => {
            let rsp = ResponseBody::<Option<AccountDTO>>::new(EMPTY_STR, Some(account), true);
            Ok(HttpResponse::build(StatusCode::OK)
                .content_type("application/json")
                .json(rsp))
        }
        Ok(None) => {
            let rsp = ResponseBody::<Option<AccountDTO>>::new("Not found", None, false);
            Ok(HttpResponse::build(StatusCode::NOT_FOUND)
                .content_type("application/json")
                .json(rsp))
        }
        Err(e) => {
            let rsp = ResponseBody::<Option<AccountDTO>>::new(e.to_string().as_str(), None, false);
            Ok(HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                .content_type("application/json")
                .json(rsp))
        }
    }
}

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
    scope.service(get_account)
}
