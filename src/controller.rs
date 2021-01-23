use actix_web::{get, post, web, Responder, HttpResponse};
use crate::service;
use crate::model::{UserCreateRequest};
use log::info;

#[get("/users")]
pub async fn get_users() -> impl Responder{
    let users = service::get_users();
    HttpResponse::Ok().json(users)
}

#[post("/users/create")]
pub async fn create_user(user_create_request: web::Json<UserCreateRequest> ) -> impl Responder{
    info!("Creating user");
    // TODO добавить реализацию отлова ошибок и бросания соответствующего кода ответа
    service::create_user(&user_create_request.name);
    HttpResponse::Created()
}