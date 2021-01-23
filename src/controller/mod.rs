pub mod user_controller;

use actix_web::{get, Responder, HttpResponse};
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct Response{
    result: bool
}

#[get("/users")]
pub async fn get_users() -> impl Responder{
    HttpResponse::Ok().json(Response{result:true})
}