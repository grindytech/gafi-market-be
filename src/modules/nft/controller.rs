use actix_web::{
    get,
    http::StatusCode,
    post,
    web::{self, Data, Json},
    Error as AWError, HttpResponse, Responder, Result,
};
use serde_json::json;

#[get("/test")]
pub async fn hello() -> impl Responder {
    format!("Hello {}!", "ss")
}

#[get("/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}
pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
    scope.service(hello).service(greet)
}
