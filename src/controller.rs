use actix_web::{get, Responder, HttpResponse};
use crate::service;

#[get("/users")]
pub async fn get_users() -> impl Responder{
    let users = service::get_users();
    HttpResponse::Ok().json(users)
}