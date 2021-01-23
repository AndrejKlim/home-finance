use actix_web::{get, Responder, HttpResponse};
use crate::service::user_service::UserService;

#[get("/users")]
pub async fn get_users() -> impl Responder{
    let mut service = UserService::new();
    let users = service.get_users();
    HttpResponse::Ok().json(users)
}