use actix_web::{
    get,
    web::{self},
    Responder,
};

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
