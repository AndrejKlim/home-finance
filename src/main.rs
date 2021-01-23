mod controller;
pub mod service;
pub mod repository;
pub mod model;

use actix_web::{App, HttpServer, web};
use log::info;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    simple_logger:: init_with_level(log::Level::Info).unwrap();

    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/serv/")
                    .service(controller::user_controller::get_users)
            )
    })
        .workers(10)
        .keep_alive(15)
        .bind("127.0.0.1:8088")?
        .run()
        .await
}